# Rustaurant

The frontend for a restaurant browsing website, built using rust's Yew framework.

The backend for the project is bulit using Axum.

# Quickstart

```
trunk serve
```
## Features
Pages: Home, Restaurant Details, About, Sign In, Sign Up

### Home Page:
- Fetches basic restaurant data from MongoDB hosted on Atlas and displays them in a grid.
- Includes a search component that queries the database based on the input string.

### Restaurant Details:
- Displays detailed information about the restaurant.
- Includes a rating component with dynamic ratings that update when a new review is posted.
- Includes a write-a-review component with user-friendly error messages (e.g., rating out of bounds).
- Added a loading animation when submitting a review from the write-a-review component.
- Added small quality of life (QOL) features such as a "Read More" button to prevent content overflow for users with smaller screens.

### About Page:
- A static page that displays the terms and services and introduces the team behind the website.

## Preview
<div style="display: flex;">
  <img src="https://github.com/Rasib0/Rustaurant/blob/master/images/1.png?raw=true" alt="Image 1" style="width: 49%;">
  <img src="https://github.com/Rasib0/Rustaurant/blob/master/images/2.png?raw=true" alt="Image 2" style="width: 49%;">
</div>
