import React, { useEffect, useState } from "react";
import { DragDropContext, Draggable, Droppable } from "react-beautiful-dnd";
import Armor from './items/armor';
import Weapon from './items/weapon';
import Jewel from './items/jewel';
import Consumable from './items/consumable';
import Spinner from 'react-bootstrap/Spinner';
import { v4 as uuidv4 } from 'uuid';

const DragToReorderList = () => {

    const [error, setError] = useState(null);
    const [isLoaded, setIsLoaded] = useState(false);
    const [columns, setColumns] = useState(null);
    const [caracter, setCaracter] = useState(null);

    const [damage, setDamage] = useState({ min: 0, max: 0 });
    const [armor, setArmor] = useState(0);
    const [resistances, setResistances] = useState({ type: '', value: 0 });

    const inventorySize = {
        width: 10,
        height: 6,
    }

    useEffect(() => {
        fetch("http://localhost:8000/api/testobjectgenerationlol/5")
            .then(res => res.json())
            .then(
                (result) => {
                    setColumns(formatResultItem(result));
                },
                (error) => {
                    setError(error);
                }
            )
    }, [])

    useEffect(() => {
        fetch("http://localhost:8000/static/images/homme2.png")
            .then(
                (result) => {
                    setCaracter(result.url)
                    setIsLoaded(true);
                },
                (error) => {
                    setError(error);
                    setIsLoaded(true);
                }
            )
    }, []);

    const formatResultItem = (items) => {

        const itemsFromBackend = [];

        for (const item of items) {
            let object;
            console.log(item);

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
    
            itemsFromBackend.push({ id: uuidv4(), content: object })
        }

        const i = 3;

        const columnsFromBackend = {
            ['Inventory']: {
                name: "Inventory",
                items: itemsFromBackend,
                style: {}
            },
            // ['Shield']: {
            //     name: "Shield",
            //     items: [],
            //     style: { margin: 8 }
            // },
            // ['RightHand']: {
            //     name: "Right hand",
            //     items: [],
            //     style: { margin: 8 }
            // },
            // ['TwoHand']: {
            //     name: "Two hands",
            //     items: [],
            //     style: { margin: 8 }
            // },
            ['Chest']: {
                name: "Chest",
                items: [],
                style: { position: "absolute", top: i + '%', marginRight: 10 + '%' }
            },
            ['Helmet']: {
                name: "Helmet",
                items: [],
                style: {position: "absolute", top: i + '%', marginRight: 70 + '%' }
            },
            ['Boot']: {
                name: "Boots",
                items: [],
                style: { position: "absolute", top: i + 12 + '%', marginRight: 10 + '%' }
            },
            ['Gloves']: {
                name: "Gloves",
                items: [],
                style: {position: "absolute", top: i + 12 + '%', marginRight: 70 + '%' }
            },
            ['Wrist']: {
                name: "Wrist",
                items: [],
                style: {position: "absolute", top: i + 24+ '%', marginRight: 10 + '%' }
            },
            ['Belt']: {
                name: "Belt",
                items: [],
                style: {position: "absolute", top: i + 24 + '%', marginRight: 70 + '%' }
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

            // if (destItems[0].content.type.name === destination.droppableId || destination.droppableId === 'Inventory'
            if (destItems[0].content.props.item.object.equipement.slot === destination.droppableId || destination.droppableId === 'Inventory'
            ) {
                if (destination.droppableId === 'Inventory') {
                    if (['Chest', 'Shield'].includes(destItems[0].content.props.item.object.equipement.slot)) {
                        //TODO: buggy
                        let value = armor - destItems[0].content.props.item.object.equipement.armor.armor
                        if(value < 0){
                            value = 0;
                        }
                        setArmor(value)
                    }

                    if (['RightHand', 'LeftHand', 'TwoHand'].includes(destItems[0].content.props.item.object.equipement.slot)) {
                        setDamage({
                            min: 0,
                            max: 0
                        })
                    }
                } else {
                    if (['RightHand', 'LeftHand', 'TwoHand'].includes(destItems[0].content.props.item.object.equipement.slot)) {
                        setDamage({
                            min: destItems[0].content.props.item.object.equipement.weapon.min_damage,
                            max: destItems[0].content.props.item.object.equipement.weapon.max_damage
                        })
                    } else if (['Chest'].includes(destItems[0].content.props.item.object.equipement.slot)) {
                        setArmor(destItems[0].content.props.item.object.equipement.armor.armor + armor)
                    } else if (['Shield'].includes(destItems[0].content.props.item.object.equipement.slot)) {
                        setArmor(destItems[0].content.props.item.object.equipement.armor.armor + armor)
                    }

                }
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

        console.log("dragged: " + source.droppableId + " into: " + destination.droppableId)
    };

    return (
        <>
            {caracter && <img src={caracter} className="caracter-img" alt="" />}
            <div style={{
                 display: "flex",
                  justifyContent: "center",
                   height: "100%"
                    }}>
                <DragDropContext
                    onDragEnd={result => onDragEnd(result, columns, setColumns)}
                >
                    {Object.entries(columns).map(([columnId, column, style], index) => {
                        return (
                            <div
                                style={{
                                    display: "flex",
                                    flexDirection: "column",
                                    alignItems: "center" 
                                }}
                                key={columnId}
                            >

                                <div style={column.style}>
                                    <h2>{column.name}</h2>
                                    <Droppable droppableId={columnId} key={columnId} direction="horizontal" >
                                        {(provided, snapshot, columnId) => {
                                            return (

                                                <div

                                                    {...provided.droppableProps}
                                                    ref={provided.innerRef}
                                                    style={{
                                                        background: snapshot.isDraggingOver
                                                            ? "lightblue"
                                                            : "lightgrey",
                                                        padding: 2,
                                                        minWidth: 100,
                                                        minHeight: 100,
                                                        // flexGrow:1,
                                                        display:"flex"
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
            <div>
                <p>Damage: min: {damage.min} max: {damage.max}</p>
                <p>Armor: {armor}</p>
                <p>Resistances: type: {resistances.type} value: {resistances.value}</p>
            </div>
        </>
    );
}
export default DragToReorderList;