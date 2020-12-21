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
                        min="10"
                        max="70"
                        value={props.strengh}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />

                    <label htmlFor="dexterity">Dexterité</label>
                    <input
                        className="form-control"
                        id="dexterity"
                        name="dexterity"
                        type="number"
                        min="10"
                        max="70"
                        value={props.dexterity}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />

                    <label htmlFor="endurance">Constitution</label>
                    <input
                        className="form-control"
                        id="endurance"
                        name="endurance"
                        type="number"
                        min="10"
                        max="70"
                        value={props.endurance}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />

                    <label htmlFor="charism">Charisme</label>
                    <input
                        className="form-control"
                        id="charism"
                        name="charism"
                        type="number"
                        min="10"
                        max="70"
                        value={props.charism}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />
                </div>

                <div class="col-md-6">
                    <label htmlFor="perception">Perception</label>
                    <input
                        className="form-control"
                        id="perception"
                        name="perception"
                        type="number"
                        min="10"
                        max="70"
                        value={props.perception}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />

                    <label htmlFor="luck">Chance</label>
                    <input
                        className="form-control"
                        id="luck"
                        name="luck"
                        type="number"
                        min="10"
                        max="70"
                        value={props.luck}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />

                    <label htmlFor="willpower">Volonté</label>
                    <input
                        className="form-control"
                        id="willpower"
                        name="willpower"
                        type="number"
                        min="10"
                        max="70"
                        value={props.willpower}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />

                    <label htmlFor="education">Education</label>
                    <input
                        className="form-control"
                        id="education"
                        name="education"
                        type="number"
                        min="10"
                        max="70"
                        value={props.education}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />
                </div>
            </div>
        </div>
    );
}

export default Step2;