# To-Let V2 API Documentation

## Overview

To-Let V2 is a CRUD API built with Rust and Actix-Web for managing property listings. The API allows you to create, read, update, and delete collections of property listings.

## Features

* **Collection Management**: Create, read, update, and delete property collections.
* **Detailed Listings**: Support for various listing types (`Single`, `Bedsitter`, `SelfContained`) and features like pricing, bedroom counts, and available units.
* **Amenity Tagging**: Categorize listings with built-in amenities such as Parking, Pools, Security, Elevators, and more.
* **CORS Enabled**: Pre-configured to communicate seamlessly with frontend applications
* **PostgreSQL Integration**: Leverages SQLx for async database queries and custom enum mapping

## Tech Stack

* **Language**: [Rust](https://www.rust-lang.org/)
* **Web Framework**: [Actix-web](https://actix.rs/)
* **Database Driver / ORM**: [SQLx](https://github.com/launchbadge/sqlx) (PostgreSQL)

## Endpoints
Base URL: http://localhost:8080

### Health Check

#### `GET /api/healthchecker`

Checks if the API is running.

**Response:**
```json
{
    "status": "success",
    "message": "Build Simple CRUD API with Rust and MongoDB"
}
```

### Collections

#### `GET /api/collections`

Fetches all collections.

**Response:**
```json
{
    "status": "success",
    "results": <number_of_collections>,
    "data": [<collection>, ...]
}
```

#### `POST /api/collections`

Creates a new collection.

**Request Body:**
```json
{
    "title": "string",
    "description": "string",
    "display_image_url": "string",
    "location": "string",
    "contact_information": "string",
    "amenities": ["Parking", "Pool", ...],
    "listings": [
        {
            "type_of_listing": "Single",
            "price": 1000,
            "number_of_bedrooms": 1,
            "available_units": 10,
            "images": ["url1", "url2"],
            "additional_fees": ["fee1", "fee2"]
        }
    ],
    "rules": ["rule1", "rule2"]
}
```

**Response:**
```json
{
    "status": "success",
    "results": <collection>
}
```

#### `PATCH /api/collections/{id}`

Updates an existing collection.

**Request Body:**
```json
{
    "title": "string",
    "description": "string",
    "display_image_url": "string",
    "location": "string",
    "contact_information": "string",
    "amenities": ["Parking", "Pool", ...],
    "listings": [
        {
            "type_of_listing": "Single",
            "price": 1000,
            "number_of_bedrooms": 1,
            "available_units": 10,
            "images": ["url1", "url2"],
            "additional_fees": ["fee1", "fee2"]
        }
    ],
    "rules": ["rule1", "rule2"]
}
```

**Response:**
```json
{
    "status": "success",
    "message": "Collection updated",
    "data": <collection>
}
```

#### `DELETE /api/collections/{id}`

Deletes a collection.

**Response:**
```json
{
    "status": "success",
    "message": "Collection deleted"
}
```

#### `GET /api/collections/{id}`

Fetches a single collection by ID.

**Response:**
```json
{
    "status": "success",
    "results": <collection>
}
```

## Data Models

### Collection

```json
{
    "id": 1,
    "title": "string",
    "display_image_url": "string",
    "description": "string",
    "location": "string",
    "contact_information": "string",
    "amenities": ["Parking", "Pool", ...],
    "listings": [
        {
            "type_of_listing": "Single",
            "price": 1000,
            "number_of_bedrooms": 1,
            "available_units": 10,
            "images": ["url1", "url2"],
            "additional_fees": ["fee1", "fee2"]
        }
    ],
    "created_at": "timestamp",
    "rules": ["rule1", "rule2"]
}
```

### Listing

```json
{
    "type_of_listing": "Single",
    "price": 1000,
    "number_of_bedrooms": 1,
    "available_units": 10,
    "images": ["url1", "url2"],
    "additional_fees": ["fee1", "fee2"]
}
```

### Amenity

```json
[
    "Parking",
    "Pool",
    "Gated",
    "Security",
    "WheelchairAccessible",
    "Elevator",
    "Electricity",
    "Water"
]
```
