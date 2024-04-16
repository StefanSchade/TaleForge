// src/components/PlayerInfo.tsx
import React from 'react';
import { TextField, Stack } from '@mui/material';
import { useDispatch } from 'react-redux';
import { setPlayerId, setGameId } from '../features/game/gameSlice'; // Import your action creators

const PlayerInfo: React.FC = () => {
    const dispatch = useDispatch();

    const handlePlayerIdChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        dispatch(setPlayerId(event.target.value));  // Dispatch action to update player ID in Redux
    };

    const handleGameIdChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        dispatch(setGameId(event.target.value));  // Dispatch action to update game ID in Redux
    };

    return (
        <Stack spacing={2}>
            <TextField
                label="Player ID"
                variant="outlined"
                onChange={handlePlayerIdChange}
            />
            <TextField
                label="Game ID"
                variant="outlined"
                onChange={handleGameIdChange}
            />
        </Stack>
    );
};

export default PlayerInfo;
