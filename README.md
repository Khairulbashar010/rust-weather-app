# Weather App

This is a simple command-line weather application written in Rust. It fetches weather data from the OpenWeatherMap API and displays it in a user-friendly format.

## Features

- Fetches current weather information for a specified city and country.
- Displays weather description, temperature, humidity, pressure, and wind speed.
- Uses colored output to enhance readability.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- OpenWeatherMap API key (https://home.openweathermap.org/users/sign_up)

## Setup

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/weather-app.git
    cd weather-app
    ```

2. Create a `.env` file in the root directory and add your OpenWeatherMap API key:
    ```sh
    OPENWEATHER_API_KEY=your_api_key_here
    ```

3. Build and run the application:
    ```sh
    cargo run
    ```

## Usage

1. Run the application:
    ```sh
    cargo run
    ```

2. Follow the prompts to enter the city name and country code.

3. The application will display the current weather information for the specified location.

4. You can choose to check the weather for another city or exit the application.

## Dependencies

- `reqwest` for making HTTP requests.
- `serde` for deserializing JSON responses.
- `colored` for colored terminal output.
- `dotenv` for loading environment variables.

## License

This project is licensed under the MIT License.
