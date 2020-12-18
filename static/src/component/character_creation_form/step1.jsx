import React from 'react';

const Step1 = (props) => {
    if (props.currentStep !== 1) {
        return null
    }
    return (
        <div className="form-group">
            <label htmlFor="name">Nom</label>
            <input
                className="form-control"
                id="name"
                name="name"
                type="text"
                placeholder="Salacious le magnifique"
                value={props.name}
                onChange={props.handleChange}
            />
            <small id="nameHelpInline" className="text-muted">
                Cela représentera le nom de votre personnage dans le jeu
            </small><br />

            <label htmlFor="age">Age</label>
            <input
                className="form-control"
                id="age"
                name="age"
                type="number"
                value={props.age}
                onChange={props.handleChange}
            />


            <label htmlFor="race">Race</label>
            <select className="form-control" name="race" onChange={props.handleRaceChange} value={props.race}>
                <option selected>Open this select menu</option>
                <option value="human">Humain</option>
                <option value="half_orc">Demi Orc</option>
                <option value="elf">Elf</option>
                <option value="half_elf">Demi elf</option>
                <option value="dwarf">Nain</option>
                <option value="saurial">Saurial</option>
                <option value="cambion">Cambion</option>
            </select>

            <label htmlFor="classs">Class</label>
            <select className="form-control" name="classs" onChange={props.handleClassChange} value={props.classs}>
                <option selected>Open this select menu</option>
                <option value="warrior">Guerrier</option>
                <option value="mage">Mage</option>
                <option value="roublard">Roublards</option>
                <option value="rodeur">Rodeur</option>
                <option value="monk">Moine</option>
                <option value="drood">Druid</option>
                <option value="paladin">Paladin</option>
                <option value="clerc">Clerc</option>
            </select>

            <label htmlFor="alignment">Alignement</label>
            <input
                className="form-control"
                id="alignment"
                name="alignment"
                type="text"
                value={props.alignment}
                onChange={props.handleChange}
            />

            <label htmlFor="particularity">Particularité</label>
            <textarea
                className="form-control"
                id="particularity"
                name="particularity"
                type="text"
                value={props.particularity}
                onChange={props.handleChange}
            />

            <label htmlFor="reputation">Réputation</label>
            <textarea
                className="form-control"
                id="reputation"
                name="reputation"
                type="number"
                value={props.reputation}
                onChange={props.handleChange}
            />
        </div>
    );
}
export default Step1;