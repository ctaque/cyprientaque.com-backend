module AddLike exposing (..)

import Browser
import Html as H exposing (Html, button, div, span, text)
import Html.Attributes as HA
import Html.Events as HE
import Http exposing (expectJson)
import Json.Decode as Decode exposing (Decoder, Error, Value, bool, decodeString, int, list, nullable, string)
import Json.Decode.Extra exposing (iso8601)
import Json.Decode.Pipeline exposing (optional, required, resolve)
import RemoteData exposing (RemoteData, WebData)
import Svg exposing (svg)
import Svg.Attributes as SvgA
import Time



-- MAIN


type alias Flags =
    { views_count : Int
    , project_id : Int
    }


main : Program Flags Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }



-- MODEL


type alias Model =
    { views_count : Int
    , project_id : Int
    , liked : Bool
    }


init : Flags -> ( Model, Cmd Msg )
init flags =
    ( { views_count = flags.views_count
      , project_id = flags.project_id
      , liked = False
      }
    , doILike flags.project_id
    )



-- UPDATE


type alias Article =
    { id : Int
    , title : String
    , content : String
    , created_at : Time.Posix
    , updated_at : Maybe Time.Posix
    , deleted_at : Maybe Time.Posix
    , images : List Image
    , user : Author
    , user_id : Int
    , category_id : Int
    , category : Category
    , slug : String
    , views_count : Int
    , likes_count : Int
    , sketchfab_model_number : String
    , is_pro : Bool
    , bitbucket_project_key : Maybe String
    }


type alias Image =
    { id : Int
    , w1500_object_url : String
    , w350_object_url : String
    , original_object_url : Maybe String
    , primary : Bool
    , views_count : Int
    , project_id : Int
    , project_image_category_id : Int
    , category : Maybe ProjectImageCategory
    }


type alias ProjectImageCategory =
    { id : Int
    , name : String
    , color_hex : String
    }


type alias ParentCategory =
    { id : Int
    , name : String
    , slug : String
    , picture_url : String
    , color_hex : String
    , project_count : Maybe Int
    }


type alias Category =
    { id : Int
    , name : String
    , slug : String
    , picture_url : String
    , color_hex : String
    , project_count : Maybe Int
    , parent_id : Int
    , parent_category : Maybe ParentCategory
    }


type alias ProfileImage =
    { id : Int
    , created_at : Time.Posix
    , user_id : Int
    , w1500_keyname : String
    , w200_keyname : String
    , w40_keyname : String
    , w1500_object_url : String
    , w200_object_url : String
    , w40_object_url : String
    }


type alias Author =
    { id : Int
    , name : String
    , profile_images : List ProfileImage
    }


decodeCategory : Decoder Category
decodeCategory =
    Decode.succeed Category
        |> required "id" int
        |> required "name" string
        |> required "slug" string
        |> required "picture_url" string
        |> required "color_hex" string
        |> optional "project_count" (nullable int) Maybe.Nothing
        |> optional "parent_id" int 0
        |> optional "parent" (nullable decodeParentCategory) Maybe.Nothing


decodeParentCategory : Decoder ParentCategory
decodeParentCategory =
    Decode.succeed ParentCategory
        |> required "id" int
        |> required "name" string
        |> required "slug" string
        |> required "picture_url" string
        |> required "color_hex" string
        |> optional "project_count" (nullable int) Maybe.Nothing


decodeCategories : Decoder (List Category)
decodeCategories =
    list decodeCategory


decodeAuthor : Decoder Author
decodeAuthor =
    Decode.succeed Author
        |> required "id" int
        |> required "name" string
        |> required "profile_images" (list decodeProfileImage)


decodeProfileImage : Decoder ProfileImage
decodeProfileImage =
    Decode.succeed ProfileImage
        |> required "id" int
        |> required "created_at" iso8601
        |> required "user_id" int
        |> required "w1500_keyname" string
        |> required "w200_keyname" string
        |> required "w40_keyname" string
        |> required "w1500_object_url" string
        |> required "w200_object_url" string
        |> required "w40_object_url" string


decodeImages : Decoder (List Image)
decodeImages =
    list decodeImage


decodeImage : Decoder Image
decodeImage =
    Decode.succeed Image
        |> required "id" int
        |> required "w1500_object_url" string
        |> required "w350_object_url" string
        |> required "original_object_url" (nullable string)
        |> required "primary" bool
        |> required "views_count" int
        |> required "project_id" int
        |> required "project_image_category_id" int
        |> optional "category" (nullable decodeProjectImageCategory) Maybe.Nothing


decodeArticle : Decoder Article
decodeArticle =
    Decode.succeed Article
        |> required "id" int
        |> required "title" string
        |> required "content" string
        |> required "created_at" iso8601
        |> required "updated_at" (nullable iso8601)
        |> required "deleted_at" (nullable iso8601)
        |> required "images" (list decodeImage)
        |> required "user" decodeAuthor
        |> required "user_id" int
        |> required "category_id" int
        |> required "category" decodeCategory
        |> required "slug" string
        |> optional "views_count" int 0
        |> optional "likes_count" int 0
        |> optional "sketchfab_model_number" string "none"
        |> required "is_pro" bool
        |> optional "bitbucket_project_key" (nullable string) Maybe.Nothing


decodeProjectImageCategory : Decoder ProjectImageCategory
decodeProjectImageCategory =
    Decode.succeed ProjectImageCategory
        |> required "id" int
        |> required "name" string
        |> required "color_hex" string


decodeDoILike : Decoder DoILike
decodeDoILike =
    Decode.succeed DoILike
        |> required "value" bool


toggleLike : Int -> Cmd Msg
toggleLike project_id =
    Http.request
        { url = "/projects/" ++ String.fromInt project_id ++ "/addLike"
        , method = "PUT"
        , headers = []
        , expect = expectJson ToggleLikeResponse decodeArticle
        , tracker = Maybe.Nothing
        , body = Http.emptyBody
        , timeout = Maybe.Nothing
        }


doILike : Int -> Cmd Msg
doILike project_id =
    Http.request
        { url = "/projects/" ++ String.fromInt project_id ++ "/doILike"
        , method = "GET"
        , headers = []
        , expect = expectJson DoILikeResponse decodeDoILike
        , tracker = Maybe.Nothing
        , body = Http.emptyBody
        , timeout = Maybe.Nothing
        }


type alias DoILike =
    { value : Bool
    }


type Msg
    = ToggleLike
    | ToggleLikeResponse (Result Http.Error Article)
    | DoILikeResponse (Result Http.Error DoILike)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        DoILikeResponse result_do_i_like ->
            case result_do_i_like of
                Ok val ->
                    ( { model | liked = val.value }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        ToggleLike ->
            ( { model | views_count = model.views_count + 1 }, toggleLike model.project_id )

        ToggleLikeResponse result_article ->
            case result_article of
                Ok article ->
                    ( { model | views_count = article.likes_count }, doILike article.id )

                _ ->
                    ( model, Cmd.none )



-- VIEW


view : Model -> Html Msg
view model =
    div [ HA.class "addLike" ]
        [ div []
            [ span [ HA.class "beforeButton" ] [ text "Was this article helpful ?" ]
            ]
        , button
            [ HE.onClick ToggleLike
            , case model.liked of
                True ->
                    HA.class "liked"

                False ->
                    HA.class "notliked"
            ]
            [ svg
                [ SvgA.viewBox "0 0 1024 1024"
                , SvgA.width "18"
                , SvgA.height "18"
                , case model.liked of
                    True ->
                        SvgA.fill "#fff"

                    False ->
                        SvgA.fill "#b98ceb"
                ]
                [ Svg.path [ SvgA.d "M912 190h-69.9c-9.8 0-19.1 4.5-25.1 12.2L404.7 724.5 207 474c-6.1-7.7-15.3-12.2-25.1-12.2H112c-6.7 0-10.4 7.7-6.3 12.9l273.9 347c12.8 16.2 37.4 16.2 50.3 0l488.4-618.9c4.1-5.1 0.4-12.8-6.3-12.8z" ] []
                ]
            , text "Yes"
            ]
        , span [ HA.class "likesCount" ] [ text (String.fromInt model.views_count ++ " people found this helpful.") ]
        ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none
