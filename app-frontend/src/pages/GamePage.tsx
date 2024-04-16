// src/pages/GamePage.tsx
import React, { useState } from 'react';
import { Container, Grid, Box } from '@mui/material';
import Room from '../components/Room';
import Navigation from '../components/Navigation';
import PlayerInfo from '../components/PlayerInfo';

interface RoomData {
    id: number;
    description: string;
    imageUrl: string;
    directions: string[];
}

// Example initial room data
const initialRoom: RoomData = {
    id: 1,
    description: 'You are in a dark, cold room. There are doors to the north and south.',
    imageUrl: 'https://via.placeholder.com/500',
    directions: ['North', 'South']
};

const GamePage: React.FC = () => {
    const [currentRoom, setCurrentRoom] = useState<RoomData>(initialRoom);
    const [playerId, setPlayerId] = useState('');
    const [gameId, setGameId] = useState('');

    const handleNavigate = (direction: string) => {
        // Placeholder for navigation logic
        console.log(`Navigate ${direction}`);
        // Here you would typically fetch new room data based on the direction
    };

    const handleSubmitPlayerInfo = (playerId: string, gameId: string) => {
        setPlayerId(playerId);
        setGameId(gameId);
        console.log(`Player ID: ${playerId}, Game ID: ${gameId}`);
        // Additional logic to handle new player or game session
    };

    return (
        <Container maxWidth="md">
            <Box my={4}>
                <Grid container spacing={4}>
                    <Grid item xs={12}>
                        <Room
                            description={currentRoom.description}
                            imageUrl={currentRoom.imageUrl}
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <Navigation
                            directions={currentRoom.directions}
                            onNavigate={handleNavigate}
                        />
                    </Grid>
                    <Grid item xs={12}>
                        <PlayerInfo
                            onSubmit={handleSubmitPlayerInfo}
                        />
                    </Grid>
                </Grid>
            </Box>
        </Container>
    );
};

export default GamePage;
