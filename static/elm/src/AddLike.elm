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
    , Cmd.none
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


type Msg
    = ToggleLike
    | ToggleLikeResponse (Result Http.Error Article)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ToggleLike ->
            ( { model | views_count = model.views_count + 1 }, toggleLike model.project_id )

        ToggleLikeResponse result_article ->
            case result_article of
                Ok article ->
                    ( { model | views_count = article.likes_count }, Cmd.none )

                _ ->
                    ( model, Cmd.none )



-- VIEW


view : Model -> Html Msg
view model =
    div [ HA.class "addLike" ]
        [ button [ HE.onClick ToggleLike ]
            [ svg
                [ SvgA.viewBox "0 0 1024 1024"
                , SvgA.width "30"
                , SvgA.height "30"
                , SvgA.fill "#ccc"
                ]
                [ Svg.path [ SvgA.d "M149.333333 607.573333v174.933334a69.12 69.12 0 0 0 0 9.386666l-26.88-26.453333A140.373333 140.373333 0 0 1 85.333333 661.76v-174.933333c0-81.066667 58.453333-136.533333 73.813334-223.573334l19.2-110.506666a21.333333 21.333333 0 0 1 24.746666-17.066667A64 64 0 0 1 256 182.613333a40.106667 40.106667 0 0 1 0 17.493334l-42.666667 249.6a234.24 234.24 0 0 0-64 157.866666zM640 115.626667a42.666667 42.666667 0 0 0-60.586667-60.586667l-170.666666 170.666667a128 128 0 0 1 40.106666 80.213333z m227.84 442.88l-85.333333 85.333333a21.333333 21.333333 0 0 1-29.866667 0 21.333333 21.333333 0 0 1 0-30.293333l175.786667-175.786667a42.666667 42.666667 0 0 0 0-60.16 42.666667 42.666667 0 0 0-60.586667 0L689.92 554.666667a21.333333 21.333333 0 0 1-29.866667 0 21.333333 21.333333 0 0 1 0-30.293334L886.613333 298.666667a42.666667 42.666667 0 0 0 0-60.16 42.666667 42.666667 0 0 0-60.586666 0l-226.56 226.56a21.333333 21.333333 0 1 1-30.293334-30.293334L768 236.373333a42.666667 42.666667 0 1 0-60.586667-60.586666L384 497.493333V317.866667A63.573333 63.573333 0 0 0 331.093333 256a21.333333 21.333333 0 0 0-24.746666 17.066667L287.146667 384C271.786667 469.333333 213.333333 526.506667 213.333333 607.573333v174.933334a140.373333 140.373333 0 0 0 37.546667 103.68l58.026667 57.6a128 128 0 0 0 90.453333 37.546666h2.133333a258.56 258.56 0 0 0 124.16-32l131.84-73.386666a216.32 216.32 0 0 0 47.36-35.84l221.44-221.013334a42.666667 42.666667 0 0 0-60.586666-60.586666z" ] []
                ]
            ]
        , span [ HA.class "likesCount" ] [ text (String.fromInt model.views_count) ]
        ]



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none
