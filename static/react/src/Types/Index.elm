module Types.Index exposing
    ( AccessToken
    , AlgoliaResponse
    , Author
    , BitbucketProject
    , BitbucketProjectLinks
    , BitbucketProjectOfRepository
    , Carousel(..)
    , Category
    , Claims
    , Env(..)
    , FilterCategory(..)
    , Image
    , ImgDimensions
    , Lang(..)
    , Link
    , Links
    , NewProject
    , ParentCategory
    , ProfileImage
    , Project
    , ProjectImageCategory
    , Repository
    , RepositoryListResponse
    , RepositoryProjectLinks
    , UpdatableProject
    , WheelDirection(..)
    )

import CssClass exposing (CssClass)
import Http
import Json.Decode as Decode exposing (Decoder, Error, Value, bool, decodeString, int, list, string)
import Json.Decode.Extra exposing (datetime)
import Json.Decode.Pipeline exposing (required, resolve)
import Time


type Env
    = Development
    | Production
    | Unknown


type Lang
    = Fr
    | En


type FilterCategory
    = Web
    | Other


type WheelDirection
    = Up
    | Down


type alias NewProject =
    { id : Maybe Int
    , title : String
    , content : String
    , category_id : Int
    , images : List Image
    }


type alias UpdatableProject =
    { id : Int
    , title : String
    , content : String
    , category_id : Int
    , user_id : Int
    }


type alias Project =
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


type alias AlgoliaResponse =
    { hits : List Project
    , nbHits : Int
    , page : Int
    , nbPages : Int
    , hitsPerPage : Int
    , processingTimeMS : Int
    , exhaustiveNbHits : Bool
    , query : String
    , params : String
    }


type alias Author =
    { id : Int
    , name : String
    , profile_images : List ProfileImage
    }


type alias ImgDimensions =
    { width : Int
    , height : Int
    }


type Carousel
    = Played
    | Paused


type alias Claims =
    { iat : Int
    , exp : Int
    , user_id : Int
    }


type alias Link =
    { href : String
    , name : Maybe String
    }


type alias Links =
    { tags : Link
    , clone : List Link
    , self : Link
    , avatar : Link
    , html : Link
    }


type alias RepositoryProjectLinks =
    { self : Link
    , avatar : Link
    }


type alias BitbucketProjectOfRepository =
    { links : RepositoryProjectLinks
    }


type alias Repository =
    { website : Maybe String
    , links : Links
    , name : String
    , project : BitbucketProjectOfRepository
    , language : String
    , created_on : Time.Posix
    , updated_on : Time.Posix
    , size : Int
    , slug : String
    , is_private : Bool
    , description : String
    }


type alias RepositoryListResponse =
    { pagelen : Int
    , size : Int
    , values : List Repository
    , page : Int
    , next : Maybe String
    , prev : Maybe String
    }


type alias BitbucketProjectLinks =
    { html : Link
    , avatar : Link
    , repositories : Link
    }


type alias BitbucketProject =
    { links : BitbucketProjectLinks
    , uuid : String
    , key : String
    , name : String
    , description : String
    , is_private : Bool
    , created_on : Time.Posix
    , updated_on : Time.Posix
    }


type alias AccessToken =
    { access_token : String
    , expired_in : Int
    , token_type : String
    , state : String
    , refresh_token : String
    }
