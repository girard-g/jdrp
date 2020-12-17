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

            <label htmlFor="age">Age</label>
            <input
                className="form-control"
                id="age"
                name="age"
                type="number"
                value={props.age}
                onChange={props.handleChange}
            />

            <label htmlFor="classs">Class</label>
            <select className="form-control" aria-label="Default select example" onChange={props.handleChange} value={props.classs}>
                <option selected>Open this select menu</option>
                <option value="1">One</option>
                <option value="2">Two</option>
                <option value="3">Three</option>
            </select>

            <label htmlFor="race">Race</label>
            <select className="form-control" aria-label="Default select example" onChange={props.handleChange} value={props.classs}>
                <option selected>Open this select menu</option>
                <option value="1">One</option>
                <option value="2">Two</option>
                <option value="3">Three</option>
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