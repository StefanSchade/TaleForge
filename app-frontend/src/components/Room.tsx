import React from 'react';
import { Card, CardMedia, CardContent, Typography } from '@mui/material';

interface RoomProps {
    description: string;
    imageUrl?: string;
}

const Room: React.FC<RoomProps> = ({ description, imageUrl }) => {
    return (
        <Card>
            {imageUrl && (
                <CardMedia
                    component="img"
                    height="140"
                    image={imageUrl}
                    alt="Room image"
                />
            )}
            <CardContent>
                <Typography variant="body2" color="text.secondary">
                    {description}
                </Typography>
            </CardContent>
        </Card>
    );
};

export default Room;
