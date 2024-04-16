// src/features/game/gameSlice.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface GameState {
    playerId: string;
    gameId: string;
}

const initialState: GameState = {
    playerId: '',
    gameId: '',
};

export const gameSlice = createSlice({
    name: 'game',
    initialState,
    reducers: {
        setPlayerId: (state, action: PayloadAction<string>) => {
            state.playerId = action.payload;
        },
        setGameId: (state, action: PayloadAction<string>) => {
            state.gameId = action.payload;
        },
    },
});

export const { setPlayerId, setGameId } = gameSlice.actions;

export default gameSlice.reducer;
