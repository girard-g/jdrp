import React, { useEffect, useState } from 'react';

const PlayerStat = props => {
    const [damage, setDamage] = useState({min:0, max: 0})
    const [armor, setArmor] = useState(0)
    const [resistances, setResistances] = useState({type: '', value: 0})

    console.log(props);

    return (
        <div>
            <p>Damage: min: {damage.min} max: {damage.max}</p>
            <p>Armor: {armor}</p>
            <p>Resistances: {resistances}</p>
        </div>
    )
}

export default  PlayerStat;