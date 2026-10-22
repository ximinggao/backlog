import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import { CreateToDoItemForm } from './components/CreateToDoItemForm';
import { ToDoItem } from './components/ToDoItem';
import { ToDoItems, isToDoItems } from './interfaces/toDoItems';
import './App.css';
import init, { rust_generate_button_text } from '../rust-interface/pkg/rust_interface.js';

const App = () => {
    const [data, setData] = useState<ToDoItems | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [wasmReady, setWasmReady] = useState<boolean>(false);
    const [RustGenerateButtonText, setRustGenerateButtonText] = useState<(input: string) => string>(null);

    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll();
            if (response.error) {
                setError(response.error);
            } else if (isToDoItems(response.data)) {
                setData(response.data);
            } else {
                setError('Unexpected response payload');
            }
        };
        if (wasmReady) {
            void fetchData();
        }
    }, [wasmReady]);

    React.useEffect(() => {
        init().then(() => {
            setRustGenerateButtonText(() => rust_generate_button_text);
            setWasmReady(true);
        }).catch(e => console.error("Error initializing WASM module:", e));
    }, []);

    function reRenderItems(response: any) {
        if (response.error) {
            alert(JSON.stringify(response));
        } else if (response.data) {
            setData(response.data);
            setError(null);
        } else {
            setError('Unknown error');
        }
    }

    let content;
    if (error) {
        content = <div style={{ color: 'red' }}>Error: {error}</div>;
    } else if (data) {
        content =
            <div className="App">
                <div className="mainContainer">
                    <div className="header">
                        <p>complete tasks: {data.done.length}</p>
                        <p>pending tasks: {data.pending.length}</p>
                    </div>
                    <h1>Pending Items</h1>
                    <div>
                        {data.pending.map((item, index) =>
                            <ToDoItem key={item.title + item.status} title={item.title} id={item.id}
                                buttonMessage={RustGenerateButtonText(item.status)} passBackResponse={reRenderItems} />
                        )}
                    </div>
                    <h1>Done Items</h1>
                    <div>
                        {data.done.map((item, index) =>
                            <ToDoItem key={item.title + item.status} title={item.title} id={item.id}
                                buttonMessage={RustGenerateButtonText(item.status)} passBackResponse={reRenderItems} />
                        )}
                    </div>
                    <CreateToDoItemForm passBackResponse={reRenderItems} />
                </div>
            </div>
    } else {
        content = <div>Loading...</div>;
    }

    return (
        <div>
            {content}
        </div>
    );
};

const rootElement = document.getElementById('root');
if (!rootElement) {
    throw new Error('Root element with id "root" was not found.');
}

const root = ReactDOM.createRoot(rootElement);
root.render(<App />);
