# Rustaurant
Where safety, performance, and great food come together on a digital plate! Bon appétit! 🍽️🦀

## Overview
Rustaurant is a restaurant browsing website built with Rust's Yew framework. It combines the power of Rust with the joy of exploring dining options.

The backend for this project is built using Axum, a Rust web framework that ensures efficient and reliable performance. You can find the backend code [here](https://github.com/Rasib0/Rustaurant-server/).

## Quickstart
To get started, simply run the following command:
```
trunk serve
```

## Features
Rustaurant offers the following key features:

### Home
- Fetches basic restaurant data from MongoDB hosted on Atlas and displays them in a grid.
- Includes a search component that allows users to query the database based on their input.

### Restaurant Details
- Provides detailed information about each restaurant.
- Includes a dynamic rating component that updates in real-time when users post new reviews.
- Features a user-friendly write-a-review component with error messages for rating validation.
- Enhancements such as a loading animation when submitting a review and a "Read More" button to prevent content overflow on smaller screens improve the user experience.

### About
- A static page that showcases the terms and services of Rustaurant.
- Introduces the talented team behind the development of this website.

## Preview
Here are a couple of preview images to give you a glimpse of Rustaurant in action:

![Image 1](https://github.com/Rasib0/Rustaurant/blob/master/images/1.png?raw=true)
![Image 2](https://github.com/Rasib0/Rustaurant/blob/master/images/2.png?raw=true)

Feel free to explore, contribute, and enjoy the flavorful journey through Rustaurant!
