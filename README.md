# Text-Based Graphic Adventure Game

This repository hosts the development of a text-based graphic adventure game, leveraging modern software architecture and coding practices. The project serves multiple purposes: exploring hexagonal architecture in Rust, experimenting with development support from AI through ChatGPT, and creating an engaging adventure game.

## Project Overview

The game invites players into a dynamic narrative where they can navigate through a richly described world, interact with objects and characters, and solve puzzles. Players experience shifting perspectives, transitioning between a vampire and a human state, which affects their abilities, the story's progression, and interactions within the game world.

## Objectives

- **Showcase Hexagonal Architecture**: Implement and explore the benefits of hexagonal architecture (ports and adapters pattern) in Rust, aiming for a clean separation of concerns, scalability, and maintainability.
- **Development with ChatGPT Support**: Document the experience and effectiveness of using ChatGPT as a development assistant, capturing insights on leveraging AI for coding assistance, architectural design, and problem-solving.
- **Create a Fun Adventure Game**: While exploring technical aspects, the primary goal is to develop an enjoyable game that offers players an immersive narrative experience.

## Architecture

- **Domain**: Contains the core game logic, including entities like players, locations, objects, and NPCs, along with the rules governing interactions and state transitions.
- **Application**: Encapsulates use cases, orchestrating the flow between the domain logic and external interfaces, ensuring the application logic is kept separate from core business rules.
- **Port**: Defines abstract interfaces for the application layer, enabling communication with external systems and services, such as data storage and user interfaces.
- **Adapter**: Implements concrete instances of port interfaces, handling data persistence, API communications, and any technology-specific details.

## Current Progress

- Setup an Actix-web server capable of handling game actions.
- Established a global mutable state for development convenience, recognizing the need for eventual refactoring towards more idiomatic Rust patterns.
- Began implementing domain logic for player movement and interaction within the game world, with plans to expand the game's narrative and mechanical depth.

## Future Directions

- Refine and expand the game's narrative and mechanical features, adding depth to player interactions and the game world.
- Transition from a global mutable state to a more robust state management solution, potentially integrating a database for persistence.
- Continue to explore and document the use of hexagonal architecture in a Rust-based project, refining the design as the project evolves.
- Enhance the development process with AI assistance, capturing best practices and lessons learned.

## Contribution

Contributions and feedback are welcome! Whether you're interested in Rust, game development, software architecture, or AI-assisted coding, your insights and contributions can help shape the project's direction.

---

This `README.md` provides a comprehensive overview of the project, its goals, and its current state, intended for collaborators, contributors, or anyone interested in the project's development journey.