// src/App.tsx
import React from 'react';
import { Outlet } from 'react-router-dom';

const App: React.FC = () => {
    return (
        <div>
            <h1>Welcome to the Adventure Game</h1>
            <Outlet /> {/* This line renders the current route's components */}
        </div>
    );
};

export default App;
