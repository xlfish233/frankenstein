use crate::games::GameHighScore;
use crate::gifts::{Gifts, OwnedGifts};
use crate::inline_mode::{PreparedInlineMessage, SentWebAppMessage};
use crate::input_file::{HasInputFile, InputFile};
use crate::input_media::{InputMedia, InputProfilePhoto, InputStoryContent, MediaGroupInputMedia};
use crate::payments::{StarAmount, StarTransactions};
use crate::response::{MessageOrBool, MethodResponse};
use crate::stickers::{Sticker, StickerSet};
use crate::types::{
    BotCommand, BotDescription, BotName, BotShortDescription, BusinessConnection,
    ChatAdministratorRights, ChatFullInfo, ChatInviteLink, ChatMember, File, ForumTopic,
    MenuButton, Message, MessageId, Poll, Story, User, UserChatBoosts, UserProfilePhotos,
};
use crate::updates::{Update, WebhookInfo};

macro_rules! request {
    ($name:ident, $return:ty) => {
        paste::paste! {
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            #[inline(always)]
            fn [<$name:snake>] (
                &self,
                params: &crate::methods::[<$name:camel Params>],
            ) -> Result<MethodResponse<$return>, Self::Error> {
                self.request(stringify!($name), Some(params))
            }
        }
    }
}

/// request no body
macro_rules! request_nb {
    ($name:ident, $return:ty) => {
        paste::paste! {
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            #[inline(always)]
            fn [<$name:snake>] (
                &self,
            ) -> Result<MethodResponse<$return>, Self::Error> {
                let params: Option<()> = None;
                self.request(stringify!($name), params)
            }
        }
    }
}

/// request with some properties utilizing [`HasInputFile`]
macro_rules! request_f {
    ($name:ident, $return:ty, $($fileproperty:ident),+) => {
        paste::paste! {
            #[doc = "Call the `" $name "` method.\n\nSee <https://core.telegram.org/bots/api#" $name:lower ">."]
            fn [<$name:snake>] (
                &self,
                params: &crate::methods::[<$name:camel Params>],
            ) -> Result<MethodResponse<$return>, Self::Error> {
                let mut files = Vec::new();
                let mut params = params.clone();
                $(
                    if let Some(file) = params.$fileproperty.replace_attach(stringify!($fileproperty)) {
                        files.push((stringify!($fileproperty).to_string(), file));
                    }
                )+
                self.request_with_possible_form_data(stringify!($name), params, files)
            }
        }
    }
}

pub trait TelegramApi {
    type Error;

    request!(getUpdates, Vec<Update>);
    request!(sendMessage, Message);
    request!(setWebhook, bool);
    request!(deleteWebhook, bool);
    request_nb!(getWebhookInfo, WebhookInfo);
    request_nb!(getMe, User);
    request_nb!(logOut, bool);
    request_nb!(close, bool);
    request!(forwardMessage, Message);
    request!(forwardMessages, Vec<MessageId>);
    request!(copyMessage, MessageId);
    request!(copyMessages, Vec<MessageId>);
    request_f!(sendPhoto, Message, photo);
    request_f!(sendAudio, Message, audio, thumbnail);

    fn send_media_group(
        &self,
        params: &crate::methods::SendMediaGroupParams,
    ) -> Result<MethodResponse<Vec<Message>>, Self::Error> {
        let mut files = Vec::new();

        macro_rules! replace_attach {
            ($base:ident. $property:ident) => {
                if let Some(file) = $base.$property.replace_attach_dyn(|| files.len()) {
                    files.push(file);
                }
            };
        }

        let mut params = params.clone();
        for media in &mut params.media {
            match media {
                MediaGroupInputMedia::Audio(audio) => {
                    replace_attach!(audio.media);
                    replace_attach!(audio.thumbnail);
                }
                MediaGroupInputMedia::Document(document) => {
                    replace_attach!(document.media);
                }
                MediaGroupInputMedia::Photo(photo) => {
                    replace_attach!(photo.media);
                }
                MediaGroupInputMedia::Video(video) => {
                    replace_attach!(video.media);
                    replace_attach!(video.cover);
                    replace_attach!(video.thumbnail);
                }
            }
        }

        self.request_with_possible_form_data("sendMediaGroup", params, files)
    }

    request_f!(sendDocument, Message, document, thumbnail);
    request_f!(sendVideo, Message, video, cover, thumbnail);
    request_f!(sendAnimation, Message, animation, thumbnail);
    request_f!(sendVoice, Message, voice);
    request_f!(sendVideoNote, Message, video_note, thumbnail);
    request!(sendPaidMedia, Message);
    request!(sendLocation, Message);
    request!(editMessageLiveLocation, MessageOrBool);
    request!(stopMessageLiveLocation, MessageOrBool);
    request!(sendChecklist, MessageOrBool);
    request!(editMessageChecklist, MessageOrBool);
    request!(sendVenue, Message);
    request!(sendContact, Message);
    request!(sendPoll, Message);
    request!(sendDice, Message);
    request!(sendChatAction, bool);
    request!(setMessageReaction, bool);
    request!(getUserProfilePhotos, UserProfilePhotos);
    request!(setUserEmojiStatus, bool);
    request!(getFile, File);
    request!(banChatMember, bool);
    request!(unbanChatMember, bool);
    request!(restrictChatMember, bool);
    request!(promoteChatMember, bool);
    request!(setChatAdministratorCustomTitle, bool);
    request!(banChatSenderChat, bool);
    request!(unbanChatSenderChat, bool);
    request!(setChatPermissions, bool);
    request!(exportChatInviteLink, String);
    request!(createChatInviteLink, ChatInviteLink);
    request!(editChatInviteLink, ChatInviteLink);
    request!(createChatSubscriptionInviteLink, ChatInviteLink);
    request!(editChatSubscriptionInviteLink, ChatInviteLink);
    request!(revokeChatInviteLink, ChatInviteLink);
    request!(approveChatJoinRequest, bool);
    request!(declineChatJoinRequest, bool);

    fn set_chat_photo(
        &self,
        params: &crate::methods::SetChatPhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let params = params.clone();
        let files = vec![("photo".to_string(), params.photo.clone())];
        self.request_with_form_data("setChatPhoto", params, files)
    }

    request!(deleteChatPhoto, bool);
    request!(setChatTitle, bool);
    request!(setChatDescription, bool);
    request!(pinChatMessage, bool);
    request!(unpinChatMessage, bool);
    request!(unpinAllChatMessages, bool);
    request!(leaveChat, bool);
    request!(getChat, ChatFullInfo);
    request!(getChatAdministrators, Vec<ChatMember>);
    request!(getChatMemberCount, u32);
    request!(getChatMember, ChatMember);
    request!(setChatStickerSet, bool);
    request!(deleteChatStickerSet, bool);
    request_nb!(getForumTopicIconStickers, Vec<Sticker>);
    request!(createForumTopic, ForumTopic);
    request!(editForumTopic, bool);
    request!(closeForumTopic, bool);
    request!(reopenForumTopic, bool);
    request!(deleteForumTopic, bool);
    request!(unpinAllForumTopicMessages, bool);
    request!(editGeneralForumTopic, bool);
    request!(closeGeneralForumTopic, bool);
    request!(reopenGeneralForumTopic, bool);
    request!(hideGeneralForumTopic, bool);
    request!(unhideGeneralForumTopic, bool);
    request!(answerCallbackQuery, bool);
    request!(getUserChatBoosts, UserChatBoosts);
    request!(getBusinessConnection, BusinessConnection);
    request!(getMyCommands, Vec<BotCommand>);
    request!(setMyCommands, bool);
    request!(deleteMyCommands, bool);
    request!(setMyName, bool);
    request!(getMyName, BotName);
    request!(setMyDescription, bool);
    request!(getMyDescription, BotDescription);
    request!(setMyShortDescription, bool);
    request!(getMyShortDescription, BotShortDescription);
    request!(answerInlineQuery, bool);
    request!(editMessageText, MessageOrBool);
    request!(editMessageCaption, MessageOrBool);

    fn edit_message_media(
        &self,
        params: &crate::methods::EditMessageMediaParams,
    ) -> Result<MethodResponse<MessageOrBool>, Self::Error> {
        let mut files = Vec::new();

        macro_rules! replace_attach {
            ($base:ident. $property:ident) => {{
                const NAME: &str = concat!(stringify!($base), "_", stringify!($property));
                if let Some(file) = $base.$property.replace_attach(NAME) {
                    files.push((NAME.to_string(), file));
                }
            }};
        }

        let mut params = params.clone();
        match &mut params.media {
            InputMedia::Animation(animation) => {
                replace_attach!(animation.media);
                replace_attach!(animation.thumbnail);
            }
            InputMedia::Document(document) => {
                replace_attach!(document.media);
                replace_attach!(document.thumbnail);
            }
            InputMedia::Audio(audio) => {
                replace_attach!(audio.media);
                replace_attach!(audio.thumbnail);
            }
            InputMedia::Photo(photo) => {
                replace_attach!(photo.media);
            }
            InputMedia::Video(video) => {
                replace_attach!(video.media);
                replace_attach!(video.cover);
                replace_attach!(video.thumbnail);
            }
        }

        self.request_with_possible_form_data("editMessageMedia", params, files)
    }

    request!(editMessageReplyMarkup, MessageOrBool);
    request!(stopPoll, Poll);
    request!(approveSuggestedPost, bool);
    request!(declineSuggestedPost, bool);
    request!(deleteMessage, bool);
    request!(deleteMessages, bool);
    request_f!(sendSticker, Message, sticker);
    request!(getStickerSet, StickerSet);

    fn upload_sticker_file(
        &self,
        params: &crate::methods::UploadStickerFileParams,
    ) -> Result<MethodResponse<File>, Self::Error> {
        let params = params.clone();
        let files = vec![("sticker".to_string(), params.sticker.clone())];
        self.request_with_form_data("uploadStickerFile", params, files)
    }

    fn create_new_sticker_set(
        &self,
        params: &crate::methods::CreateNewStickerSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let mut files = Vec::new();

        let mut params = params.clone();
        for (index, sticker) in params.stickers.iter_mut().enumerate() {
            if let Some(file) = sticker.sticker.replace_attach_dyn(|| index) {
                files.push(file);
            }
        }

        self.request_with_possible_form_data("createNewStickerSet", params, files)
    }

    request!(getCustomEmojiStickers, Vec<Sticker>);

    fn add_sticker_to_set(
        &self,
        params: &crate::methods::AddStickerToSetParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let mut files = Vec::new();
        let mut params = params.clone();
        if let Some(file) = params.sticker.sticker.replace_attach("sticker_upload") {
            files.push(("sticker_upload".to_string(), file));
        }
        self.request_with_possible_form_data("addStickerToSet", params, files)
    }

    request!(setStickerPositionInSet, bool);
    request!(deleteStickerFromSet, bool);
    request!(replaceStickerInSet, bool);
    request!(setStickerEmojiList, bool);
    request!(setStickerKeywords, bool);
    request!(setStickerMaskPosition, bool);
    request!(setStickerSetTitle, bool);
    request_f!(setStickerSetThumbnail, bool, thumbnail);
    request!(setCustomEmojiStickerSetThumbnail, bool);
    request!(deleteStickerSet, bool);
    request_nb!(getAvailableGifts, Gifts);
    request!(sendGift, bool);
    request!(giftPremiumSubscription, bool);
    request!(verifyUser, bool);
    request!(verifyChat, bool);
    request!(removeUserVerification, bool);
    request!(removeChatVerification, bool);
    request!(readBusinessMessage, bool);
    request!(deleteBusinessMessages, bool);
    request!(setBusinessAccountName, bool);
    request!(setBusinessAccountUsername, bool);
    request!(setBusinessAccountBio, bool);

    fn set_business_account_profile_photo(
        &self,
        params: &crate::methods::SetBusinessAccountProfilePhotoParams,
    ) -> Result<MethodResponse<bool>, Self::Error> {
        let mut files = Vec::new();

        let mut params = params.clone();
        match &mut params.photo {
            InputProfilePhoto::Static(photo_static) => {
                if let Some(file) = photo_static.photo.replace_attach("photo_static") {
                    files.push(("photo_static".to_string(), file));
                }
            }
            InputProfilePhoto::Animated(photo_animated) => {
                if let Some(file) = photo_animated.animation.replace_attach("photo_animated") {
                    files.push(("photo_animated".to_string(), file));
                }
            }
        }

        self.request_with_possible_form_data("setBusinessAccountProfilePhoto", params, files)
    }

    request!(removeBusinessAccountProfilePhoto, bool);
    request!(setBusinessAccountGiftSettings, bool);
    request!(getBusinessAccountStarBalance, StarAmount);
    request!(transferBusinessAccountStars, bool);
    request!(getBusinessAccountGifts, OwnedGifts);
    request!(convertGiftToStars, bool);
    request!(upgradeGift, bool);
    request!(transferGift, bool);

    fn post_story(
        &self,
        params: &crate::methods::PostStoryParams,
    ) -> Result<MethodResponse<Story>, Self::Error> {
        let mut files = Vec::new();

        let mut params = params.clone();

        match &mut params.content {
            InputStoryContent::Photo(photo_content) => {
                if let Some(file) = photo_content.photo.replace_attach("photo_content") {
                    files.push(("photo_content".to_string(), file));
                }
            }
            InputStoryContent::Video(video_content) => {
                if let Some(file) = video_content.video.replace_attach("video_content") {
                    files.push(("video_content".to_string(), file));
                }
            }
        }

        self.request_with_possible_form_data("postStory", params, files)
    }

    fn edit_story(
        &self,
        params: &crate::methods::EditStoryParams,
    ) -> Result<MethodResponse<Story>, Self::Error> {
        let mut files = Vec::new();

        let mut params = params.clone();

        match &mut params.content {
            InputStoryContent::Photo(photo_content) => {
                if let Some(file) = photo_content.photo.replace_attach("photo_content") {
                    files.push(("photo_content".to_string(), file));
                }
            }
            InputStoryContent::Video(video_content) => {
                if let Some(file) = video_content.video.replace_attach("video_content") {
                    files.push(("video_content".to_string(), file));
                }
            }
        }

        self.request_with_possible_form_data("editStory", params, files)
    }

    request!(deleteStory, bool);
    request!(sendInvoice, Message);
    request!(createInvoiceLink, String);
    request!(answerShippingQuery, bool);
    request!(answerPreCheckoutQuery, bool);
    request_nb!(getMyStarBalance, u32);
    request!(getStarTransactions, StarTransactions);
    request!(refundStarPayment, bool);
    request!(editUserStarSubscription, bool);
    request!(sendGame, Message);
    request!(setGameScore, MessageOrBool);
    request!(getGameHighScores, Vec<GameHighScore>);
    request!(setMyDefaultAdministratorRights, bool);
    request!(getMyDefaultAdministratorRights, ChatAdministratorRights);
    request!(answerWebAppQuery, SentWebAppMessage);
    request!(savePreparedInlineMessage, PreparedInlineMessage);
    request!(setChatMenuButton, bool);
    request!(getChatMenuButton, MenuButton);
    request!(unpinAllGeneralForumTopicMessages, bool);
    request!(setPassportDataErrors, bool);

    fn request_with_possible_form_data<Params, Output>(
        &self,
        method_name: &str,
        params: Params,
        files: Vec<(String, InputFile)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned,
    {
        if files.is_empty() {
            self.request(method_name, Some(params))
        } else {
            self.request_with_form_data(method_name, params, files)
        }
    }

    fn request<Params, Output>(
        &self,
        method: &str,
        params: Option<Params>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned;

    fn request_with_form_data<Params, Output>(
        &self,
        method: &str,
        params: Params,
        files: Vec<(String, InputFile)>,
    ) -> Result<Output, Self::Error>
    where
        Params: serde::ser::Serialize + std::fmt::Debug,
        Output: serde::de::DeserializeOwned;
}
