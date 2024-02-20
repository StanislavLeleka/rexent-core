# rexent-core

⚠️ Important: This project is currently on pause and incomplete.

# Overview

This repo represents an ambitious project aimed at providing personalized recommendations to users based on their context. While the ultimate goal is to offer a variety of recommendations across different domains, the project is currently focused solely on movie recommendations.

# Current Status

As of now, the project is incomplete and has been put on pause. While significant progress has been made, especially in the area of movie recommendations, other aspects of the project are yet to be developed.

# Components

## Services
### Users
The `users` service handles basic user management tasks such as sign-up and sign-in processes. Additionally, it manages user-specific data like favorite movies.
#### Key Features
**User Management**: Supports user registration, authentication, and profile management.<br/>
**Favorite Movies Management**: Allows users to maintain a list of their favorite movies.

### Search
The `search` service currently focuses on movie search functionality, offering robust search capabilities within the movie domain.
#### Key Features
**Movie Search**: Provides a comprehensive search feature for movies, enabling users to find movies based on various criteria.

### Reco
`reco` is dedicated to managing user preferences for different types of recommendations. Currently, it offers movie recommendation services based on user preferences.
#### Key Features
**User Preference Management**: Stores and manages individual user preferences for recommendations.<br/>
**Movie Recommendations**: Generates movie recommendations tailored to the user's tastes.

### Location
The `location` service is responsible for managing user locations. It triggers location-based events, currently focusing on updating now playing movies and showtimes based on user location.
#### Key Features
**Location Management**: Keeps track of user locations.<br/>
**Location-Based Event Triggering**: Initiates updates for movies and showtimes based on the user's location.

### Gateway
`gateway` acts as the entry point, providing a gateway to various publicly accessible APIs.
#### Key Features
**API Gateway**: Centralizes access to various services, offering a single point of entry for external requests.

### Events
The `events` service offers information about various events, currently focusing on movie-related events, based on user location.
##### Key Features
**Event Information**: Provides details about different location-based events, primarily in the movie domain.

### Context
`context` represents the current context of a user, including factors like location, mood, activity, and weather, to enhance user experience and service personalization.
#### Key Features
`User Context Representation`: Captures and utilizes various aspects of a user's current situation to tailor services and recommendations.

## Service-Protos
The `service-protos` project is dedicated to defining and managing the Protocol Buffers (protobuf) for internal service-to-service communication. This centralized approach ensures consistency and efficiency in the way services interact with each other.
### Key Features
**Protobuf Definitions**: Contains all the protobuf definitions required for internal service requests, ensuring a standard and efficient protocol for inter-service communication. <br />
**Inter-Service Communication**: Facilitates streamlined and consistent communication protocols between different services. <br />
**Scalability and Maintenance**: By centralizing the protobuf definitions, the `service-protos` project makes it easier to maintain and scale the communication protocols as the number of services grows or their requirements change.

### Usage
Services these protobuf definitions to ensure uniformity in communication. <br />
Any changes or updates to inter-service communication protocols should be reflected in this project.

## Tools
### Data-Import
The `data-import` tool is designed for importing data from CSV files into the database. This utility is essential for initializing or updating the database with bulk data efficiently.
#### Key Features
**CSV to Database Import**: Facilitates the transfer of data from CSV files directly into the database.<br/>
**Data Initialization and Updates**: Useful for both initial data setup and subsequent updates or migrations.

### Events-Crawler
The `events-crawler` tool specializes in crawling and gathering showtime information for movies based on specific locations. It plays a pivotal role in keeping the showtime data up-to-date and accurate.
#### Key Features
**Showtime Crawling**: Efficiently collects detailed showtime information for movies in various locations. <br />
**Location-Specific Data Retrieval**: Tailors the crawling process to fetch data relevant to specified locations.

## Libraries
### API-Error
The `api-error` library is a wrapper designed to handle errors from various dependencies like bcrypt, diesel, and jsonwebtoken. It efficiently converts these errors into HTTP responses.

#### Key Features
**Error Handling**: Simplifies the management of errors from different sources. <br />
**HTTP Response Integration**: Converts various errors into standardized HTTP responses for consistency in API communication.

### Auth
The `auth` library serves as an Actix middleware specifically for JWT (JSON Web Token) authentication, enhancing security and access control in applications.

#### Key Features
**JWT Integration**: Implements JWT authentication as middleware for Actix web applications. <br />
Access Control: Provides a mechanism for secure and controlled access to resources.

### DB-Connector
`db-connector` offers foundational setup for database connections and repositories, supporting both MongoDB and PostgreSQL databases.

#### Key Features
**Dual Database Support**: Facilitates connections to both MongoDB and PostgreSQL databases. <br />
**Repository Setup**: Provides a base setup for database repositories, enhancing the ease of database operations.

### HTTP-Client
The `http-client` library is a wrapper for making HTTP requests, simplifying the process of external communication in applications.
Key Features

### Movie-Fetcher
`movie-fetcher` defines the logic for fetching currently playing movies based on stored locations, and it's utilized in the cron-movie-fetcher service.

#### Key Features
**Movie Data Retrieval**: Fetches data about now-playing movies from specified locations. <br />
Integration: Key component of the cron-movie-fetcher service for regular data updates.

### Movie-Reco-Client
This library, `movie-reco-client`, wraps requests to a movie recommender service, facilitating the integration of movie recommendations into applications.

### TMDB-Client
The `tmdb-client` is a specialized API client for interacting with the TMDB (The Movie Database) API.

### WeatherAPI-Client
`weatherapi-client` serves as a client for interfacing with the WeatherAPI, allowing applications to access weather-related data.

## Jobs
### Cron-Movie-Fetcher
The Cron-Movie-Fetcher is a specialized cron job service designed to automate the task of fetching information about movies currently playing in a specific location. This service is triggered according to a predefined schedule, ensuring timely and regular updates.
Functionality

### Cron-Showtime-Crawler
The Cron-Showtime-Crawler is an automated service designed for crawling and gathering showtime information for movies in specified locations. Like the Cron-Movie-Fetcher, this service operates on a scheduled basis, ensuring that showtime data is consistently up-to-date.