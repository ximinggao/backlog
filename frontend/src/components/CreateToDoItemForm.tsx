import React, { useState } from 'react';
import { createToDoItemCall } from '../api/create';

interface CreateItemFormProps {
    passBackResponse: (response: any) => void;
}

export const CreateToDoItemForm: React.FC<CreateItemFormProps> = ({ passBackResponse }) => {
    const [title, setTitle] = useState('');
    const handleTitleChange = (event: React.ChangeEvent<HTMLInputElement>) => { setTitle(event.target.value); };
    const createItem = async () => {
        await createToDoItemCall(title).then((response) => {
            setTitle('');
            passBackResponse(response);
        });
    };

    return (
        <div className="inputContainer">
            <input type="text" id="name" placeholder="creat to do item" value={title} onChange={handleTitleChange} />
            <button className="actionButton" id="create-button" onClick={createItem}>Create</button>
        </div>
    );
}
