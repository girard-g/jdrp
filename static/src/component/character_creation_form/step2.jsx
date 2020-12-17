import React from 'react';

const Step2 = (props) => {
    if (props.currentStep !== 2) {
        return null
    }

    return (

        <div className="form-group">
            <div className='row'>
                <div class="col-md-6">

                    <label htmlFor="strengh">Force</label>
                    <input
                        className="form-control"
                        id="strengh"
                        name="strengh"
                        type="number"
                        value={props.strengh}
                        onChange={props.handleChange}
                    />

                    <label htmlFor="dexterity">Dexterité</label>
                    <input
                        className="form-control"
                        id="dexterity"
                        name="dexterity"
                        type="number"
                        value={props.dexterity}
                        onChange={props.handleChange}
                    />

                    <label htmlFor="endurance">Constitution</label>
                    <input
                        className="form-control"
                        id="endurance"
                        name="endurance"
                        type="number"
                        value={props.endurance}
                        onChange={props.handleChange}
                    />

                    <label htmlFor="charism">Charisme</label>
                    <input
                        className="form-control"
                        id="charism"
                        name="charism"
                        type="number"
                        value={props.charism}
                        onChange={props.handleChange}
                    />
                </div>

                <div class="col-md-6">
                    <label htmlFor="perception">Perception</label>
                    <input
                        className="form-control"
                        id="perception"
                        name="perception"
                        type="number"
                        value={props.perception}
                        onChange={props.handleChange}
                    />

                    <label htmlFor="luck">Chance</label>
                    <input
                        className="form-control"
                        id="luck"
                        name="luck"
                        type="number"
                        value={props.luck}
                        onChange={props.handleChange}
                    />

                    <label htmlFor="willpower">Volonté</label>
                    <input
                        className="form-control"
                        id="willpower"
                        name="willpower"
                        type="number"
                        value={props.willpower}
                        onChange={props.handleChange}
                    />

                    <label htmlFor="education">Education</label>
                    <input
                        className="form-control"
                        id="education"
                        name="education"
                        type="number"
                        value={props.education}
                        onChange={props.handleChange}
                    />
                </div>
            </div>
        </div>
    );
}

export default Step2;