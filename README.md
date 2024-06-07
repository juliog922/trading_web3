# Anonymous Trading App with Blockchain and Token System

## Overview
Welcome to the Anonymous Trading App, an innovative trading platform designed with user anonymity, security, and blockchain integration in mind. Our app allows users to trade stocks without the need for traditional email or mobile registration. Instead, we leverage Zero-Knowledge Proofs (ZKP) and mnemonic phrases to ensure secure and private user authentication. Additionally, the app includes a custom blockchain and token system, providing users with unique trading points and ranking based on their performance. An AI chat feature will also be available to offer investment recommendations.

## Features
- **Anonymous Registration:** Users can register with just a username, no email or mobile number required.
- **Secure Authentication:** User authentication is handled through JWT, ZKP, and mnemonic phrases, ensuring no passwords are stored.
- **Simple Frontend:** Built with Yew for a minimalistic and user-friendly interface, allowing users to buy, sell, and place stop and take orders on various stocks through a simple table view.
- **Backend Services:** Developed with `actix_web` and connects to the Alpaca API via WebSocket to provide real-time trading data.
- **Microservices Architecture:** Utilizes Docker to manage backend, frontend, crypto_service, and blockchain components.
- **Blockchain Integration:** The app runs on a custom blockchain that records all user transactions and features its own token system. Users can earn tokens and trade points based on their trading performance.
- **Token and Staking System:** Our PoS blockchain allows for staking of tokens, which can be exchanged for trading points. Smart contracts manage trading operations, user transfers, and trading point purchases.
- **AI Chat for Investment Recommendations:** An AI-powered chat feature will provide users with investment advice and recommendations.

## Architecture
- **Frontend:** Built with Yew for a lightweight and easy-to-use interface.
- **Backend:** Developed with `actix_web` for handling user authentication, trading operations, and data fetching from Alpaca API.
- **Crypto Service:** A gRPC service that handles cryptographic operations including ZKP and mnemonic phrase generation.
- **Database:** User data is stored securely in a PostgreSQL database.
- **Blockchain:** Custom-built blockchain for transaction recording, token management, and staking.
- **Microservices:** All components run as Docker containers for ease of deployment and scalability.

## Components
1. **Frontend Service**
   - Developed with Yew.
   - Simple interface for trading operations.
2. **Backend Service**
   - Developed with `actix_web`.
   - Handles user login, trading operations, and connects to Alpaca API.
3. **Crypto Service**
   - gRPC service for cryptographic operations.
   - Manages ZKP and mnemonic phrases.
4. **Blockchain Service**
   - Custom blockchain for transaction recording and token management.
   - Supports PoS staking and smart contracts.
5. **AI Chat Service**
   - Provides investment recommendations and advice.

## Deployment and CI/CD
- **CI/CD Pipeline:** To be defined.
- **Deployment Platform:** To be defined.

## Getting Started
1. Clone the repository.
2. Set up Docker on your system.
3. Build and run the Docker containers for each service.
4. Follow the instructions in the `docker-compose.yml` to start all services.

## Future Work
- Define and implement the node/validator registration system, likely using Docker-based validators.
- Finalize the CI/CD pipeline and choose a deployment platform.

## Contributing
We welcome contributions! Please fork the repository and submit a pull request with your changes.
