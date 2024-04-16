import React from 'react';
import { Button, Stack } from '@mui/material';

interface NavigationProps {
    directions: string[];
    onNavigate: (direction: string) => void;
}

const Navigation: React.FC<NavigationProps> = ({ directions, onNavigate }) => {
    return (
        <Stack direction="row" spacing={2}>
            {directions.map((direction) => (
                <Button
                    key={direction}
                    variant="contained"
                    onClick={() => onNavigate(direction)}
                >
                    {direction}
                </Button>
            ))}
        </Stack>
    );
};

export default Navigation;
