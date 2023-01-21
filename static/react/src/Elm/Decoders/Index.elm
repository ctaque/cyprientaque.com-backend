module Elm.Decoders.Index exposing
    ( decodeAccessToken
    , decodeAlgoliaResponse
    , decodeAuthor
    , decodeBitbucketProject
    , decodeBitbucketRepositoryResponse
    , decodeCategories
    , decodeCategory
    , decodeIds
    , decodeImage
    , decodeImageLoaded
    , decodeImages
    , decodeProject
    , decodeProjects
    , envDecoder
    , filterCategoryDecoder
    , jwtTokenDecoder
    , langDecoder
    , slugDecoder
    )

import Json.Decode as Decode exposing (Decoder, Error, Value, bool, decodeString, int, list, nullable, string)
import Json.Decode.Extra exposing (datetime)
import Json.Decode.Pipeline exposing (optional, required, resolve)
import Json.Encode as Encode
import Types.Index
    exposing
        ( AccessToken
        , AlgoliaResponse
        , Author
        , BitbucketProject
        , BitbucketProjectLinks
        , BitbucketProjectOfRepository
        , Category
        , Env(..)
        , FilterCategory(..)
        , Image
        , ImgDimensions
        , Lang(..)
        , Link
        , Links
        , ParentCategory
        , ProfileImage
        , Project
        , ProjectImageCategory
        , Repository
        , RepositoryListResponse
        , RepositoryProjectLinks
        )


langDecoder : Decoder Lang
langDecoder =
    Decode.field "lang" Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "fr" ->
                        Decode.succeed Fr

                    _ ->
                        Decode.succeed En
            )


decodeAlgoliaResponse : Decoder AlgoliaResponse
decodeAlgoliaResponse =
    Decode.succeed AlgoliaResponse
        |> required "hits" (list decodeProject)
        |> required "nbHits" int
        |> required "page" int
        |> required "nbPages" int
        |> required "hitsPerPage" int
        |> required "processingTimeMS" int
        |> required "exhaustiveNbHits" bool
        |> required "query" string
        |> required "params" string


decodeProject : Decoder Project
decodeProject =
    Decode.succeed Project
        |> required "id" int
        |> required "title" string
        |> required "content" string
        |> required "created_at" datetime
        |> required "updated_at" (nullable datetime)
        |> required "deleted_at" (nullable datetime)
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


decodeProjects : Decoder (List Project)
decodeProjects =
    list decodeProject


decodeIds : Decoder (List Int)
decodeIds =
    list Decode.int


decodeParentCategory : Decoder ParentCategory
decodeParentCategory =
    Decode.succeed ParentCategory
        |> required "id" int
        |> required "name" string
        |> required "slug" string
        |> required "picture_url" string
        |> required "color_hex" string
        |> optional "project_count" (nullable int) Maybe.Nothing


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


decodeCategories : Decoder (List Category)
decodeCategories =
    list decodeCategory


decodeAuthor : Decoder Author
decodeAuthor =
    Decode.succeed Author
        |> required "id" int
        |> required "name" string
        |> required "profile_images" (list decodeProfileImage)


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


decodeProjectImageCategory : Decoder ProjectImageCategory
decodeProjectImageCategory =
    Decode.succeed ProjectImageCategory
        |> required "id" int
        |> required "name" string
        |> required "color_hex" string


decodeProfileImage : Decoder ProfileImage
decodeProfileImage =
    Decode.succeed ProfileImage
        |> required "id" int
        |> required "created_at" datetime
        |> required "user_id" int
        |> required "w1500_keyname" string
        |> required "w200_keyname" string
        |> required "w40_keyname" string
        |> required "w1500_object_url" string
        |> required "w200_object_url" string
        |> required "w40_object_url" string


decodeImageLoaded msg =
    Decode.map msg <|
        Decode.field "target" <|
            Decode.map2 ImgDimensions
                (Decode.field "width" Decode.int)
                (Decode.field "height" Decode.int)


filterCategoryDecoder : Decoder FilterCategory
filterCategoryDecoder =
    Decode.field "activeCategory"
        Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "software" ->
                        Decode.succeed Web

                    "wood" ->
                        Decode.succeed Other

                    _ ->
                        Decode.succeed Other
            )


envDecoder : Decoder Env
envDecoder =
    Decode.field "env"
        Decode.string
        |> Decode.andThen
            (\str ->
                case str of
                    "development" ->
                        Decode.succeed Development

                    "production" ->
                        Decode.succeed Production

                    _ ->
                        Decode.succeed Unknown
            )


jwtTokenDecoder : Decoder String
jwtTokenDecoder =
    Decode.field "jwtToken" Decode.string


slugDecoder : Decoder String
slugDecoder =
    Decode.field "slug" Decode.string


decodeLink : Decoder Link
decodeLink =
    Decode.succeed Link
        |> required "href" string
        |> optional "name" (nullable string) Maybe.Nothing


decodeRepository : Decoder Repository
decodeRepository =
    Decode.succeed
        Repository
        |> optional "website" (nullable string) Maybe.Nothing
        |> required "links"
            (Decode.succeed Links
                |> required "tags" decodeLink
                |> required "clone" (list decodeLink)
                |> required "self" decodeLink
                |> required "avatar" decodeLink
                |> required "html" decodeLink
            )
        |> required "name" string
        |> required "project"
            (Decode.succeed BitbucketProjectOfRepository
                |> required "links"
                    (Decode.succeed RepositoryProjectLinks
                        |> required "self" decodeLink
                        |> required "avatar" decodeLink
                    )
            )
        |> required "language" string
        |> required "created_on" datetime
        |> required "updated_on" datetime
        |> required "size" int
        |> required "slug" string
        |> required "is_private" bool
        |> required "description" string


decodeBitbucketRepositoryResponse : Decoder RepositoryListResponse
decodeBitbucketRepositoryResponse =
    Decode.succeed RepositoryListResponse
        |> required "pagelen" int
        |> required "size" int
        |> required "values" (list decodeRepository)
        |> required "page" int
        |> optional "next" (nullable string) Maybe.Nothing
        |> optional "prev" (nullable string) Maybe.Nothing


decodeBitbucketProject : Decoder BitbucketProject
decodeBitbucketProject =
    Decode.succeed BitbucketProject
        |> required "links"
            (Decode.succeed BitbucketProjectLinks
                |> required "html" decodeLink
                |> required "avatar" decodeLink
                |> required "repositories" decodeLink
            )
        |> required "uuid" string
        |> required "key" string
        |> required "name" string
        |> required "description" string
        |> required "is_private" bool
        |> required "created_on" datetime
        |> required "updated_on" datetime


decodeAccessToken : Decoder AccessToken
decodeAccessToken =
    Decode.succeed AccessToken
        |> required "access_token" string
        |> required "expires_in" int
        |> required "token_type" string
        |> required "state" string
        |> required "refresh_token" string
