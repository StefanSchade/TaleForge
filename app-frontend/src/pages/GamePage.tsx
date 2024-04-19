// src/pages/GamePage.tsx
import React, { useState } from 'react';
import { Container, Grid, Box } from '@mui/material';
import Room from '../components/Room';
import Navigation from '../components/Navigation';
import PlayerInfo from '../components/PlayerInfo';
import { useSelector } from 'react-redux';
import { RootState } from '../app/store'; // Update path as necessary

interface RoomData {
    id: number;
    description: string;
    imageUrl: string;
    directions: string[];
}

const initialRoom: RoomData = {
    id: 1,
    description: 'You are in a dark, cold room. There are doors to the north and south.',
    imageUrl: 'https://via.placeholder.com/500',
    directions: ['North', 'South']
};

const GamePage: React.FC = () => {
    const [currentRoom] = useState<RoomData>(initialRoom);
    // Select playerId and gameId from Redux store
    const playerId = useSelector((state: RootState) => state.game.playerId);
    const gameId = useSelector((state: RootState) => state.game.gameId);

    const handleNavigate = (direction: string) => {
        console.log(`Navigate ${direction} with Player ID: ${playerId} and Game ID: ${gameId}`);
        // Your navigation logic here, using playerId and gameId as context
    };

    return (
        <Container maxWidth="md">
            <Box my={4}>
                <Grid container spacing={4}>
                    <Grid item xs={12}>
                        <PlayerInfo />
                    </Grid>
                    <Grid item xs={12}>
                        <Room description={currentRoom.description} imageUrl={currentRoom.imageUrl} />
                    </Grid>
                    <Grid item xs={12}>
                        <Navigation directions={currentRoom.directions} onNavigate={handleNavigate} />
                    </Grid>
                </Grid>
            </Box>
        </Container>
    );
};

export default GamePage;
