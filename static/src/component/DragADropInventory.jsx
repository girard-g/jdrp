import React, { useEffect, useState } from "react";
import { DragDropContext, Draggable, Droppable } from "react-beautiful-dnd";
import Armor from './items/armor';
import Weapon from './items/weapon';
import Jewel from './items/jewel';
import Consumable from './items/consumable';
import Spinner from 'react-bootstrap/Spinner';

const DragToReorderList = () => {

    const [error, setError] = useState(null);
    const [isLoaded, setIsLoaded] = useState(false);
    const [columns, setColumns] = useState(null);

    const inventorySize = {
        width: 10,
        height: 6,
    }

    useEffect(() => {
        fetch("http://localhost:8000/api/testobjectgenerationlol")
            .then(res => res.json())
            .then(
                (result) => {
                    setColumns(formatResultItem(result));
                    setIsLoaded(true);
                },
                (error) => {
                    setError(error);
                    setIsLoaded(true);
                }
            )
    }, [])

    const formatResultItem = (item) => {

        let object;

        if (item.object !== null) {
            if (item.object.equipement.armor !== null) {
                object = <Armor item={item} />
            } else if (item.object.equipement.weapon !== null) {
                object = <Weapon item={item} />
            } else {
                object = <Jewel item={item} />
            }
        } else {
            object = <Consumable item={item} />
        }

        const itemsFromBackend = [
            { id: 'qwe', content: object },
        ];

        const columnsFromBackend = {
            ['Inventory']: {
                name: "Inventory",
                items: itemsFromBackend
            },
            ['Equipped']: {
                name: "Equipped",
                items: []
            },
        };

        return columnsFromBackend
    }


    if (!isLoaded) return <Spinner animation="border" role="status" variant="primary"><span className="sr-only">Loading...</span></Spinner>
    if (error) return <div>Erreur : {error.message}</div>
    if (columns === null) return <Spinner animation="border" role="status" variant="primary"><span className="sr-only">Loading...</span></Spinner>

    const onDragEnd = (result, columns, setColumns) => {
        if (!result.destination) return;
        const { source, destination } = result;

        if (source.droppableId !== destination.droppableId) {
            const sourceColumn = columns[source.droppableId];
            const destColumn = columns[destination.droppableId];
            const sourceItems = [...sourceColumn.items];
            const destItems = [...destColumn.items];
            const [removed] = sourceItems.splice(source.index, 1);
            destItems.splice(destination.index, 0, removed);
            setColumns({
                ...columns,
                [source.droppableId]: {
                    ...sourceColumn,
                    items: sourceItems
                },
                [destination.droppableId]: {
                    ...destColumn,
                    items: destItems
                }
            });
        } else {
            const column = columns[source.droppableId];
            const copiedItems = [...column.items];
            const [removed] = copiedItems.splice(source.index, 1);
            copiedItems.splice(destination.index, 0, removed);
            setColumns({
                ...columns,
                [source.droppableId]: {
                    ...column,
                    items: copiedItems
                }
            });
        }
    };



    // const [inventorySize] = useState(inventorySize);
    return (
        <div style={{ display: "flex", justifyContent: "center", height: "100%" }}>
            <DragDropContext
                onDragEnd={result => onDragEnd(result, columns, setColumns)}
            >
                {Object.entries(columns).map(([columnId, column], index) => {
                    return (
                        <div
                            style={{
                                display: "flex",
                                flexDirection: "column",
                                alignItems: "center"
                            }}
                            key={columnId}
                        >
                            <h2>{column.name}</h2>
                            <div style={{ margin: 8 }}>
                                <Droppable droppableId={columnId} key={columnId}>
                                    {(provided, snapshot) => {
                                        return (
                                            <div
                                                {...provided.droppableProps}
                                                ref={provided.innerRef}
                                                style={{
                                                    background: snapshot.isDraggingOver
                                                        ? "lightblue"
                                                        : "lightgrey",
                                                    padding: 2,
                                                    width: 150,
                                                    minHeight: 500
                                                }}
                                            >
                                                {column.items.map((item, index) => {
                                                    return (
                                                        <Draggable
                                                            key={item.id}
                                                            draggableId={item.id}
                                                            index={index}
                                                        >
                                                            {(provided, snapshot) => {
                                                                return (
                                                                    <div
                                                                        ref={provided.innerRef}
                                                                        {...provided.draggableProps}
                                                                        {...provided.dragHandleProps}
                                                                        style={{
                                                                            userSelect: "none",
                                                                            padding: 0,
                                                                            margin: "0 0 8px 0",
                                                                            minHeight: "50px",
                                                                            backgroundColor: snapshot.isDragging
                                                                                ? "#263B4A"
                                                                                : "#456C86",
                                                                            color: "white",
                                                                            ...provided.draggableProps.style
                                                                        }}
                                                                    >
                                                                        {item.content}
                                                                    </div>
                                                                );
                                                            }}
                                                        </Draggable>
                                                    );
                                                })}
                                                {provided.placeholder}
                                            </div>
                                        );
                                    }}
                                </Droppable>
                            </div>
                        </div>
                    );
                })}
            </DragDropContext>
        </div>
    );
}
export default DragToReorderList;