import React from 'react';

const Step3 = (props) => {
    if (props.currentStep !== 3) {
        return null
    }

    return (
        <div className="form-group">
            <div className="container overflow-auto " style={{ height: 600, maxHeight: 600 }}>

                <div className="row">
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait1" name="portrait" value="portrait1" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait2" name="portrait" value="portrait2" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                </div>
                <br />

                <div className="row">
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait3" name="portrait" value="portrait3" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait4" name="portrait" value="portrait4" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                </div>
                <br />

                <div className="row">
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait5" name="portrait" value="portrait5" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait6" name="portrait" value="portrait6" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                </div>
                <br />
                
                <div className="row">
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait7" name="portrait" value="portrait7" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                    <div class="col-md-6" >
                        <label class="form-check-label">
                            <input class="form-check-input" type="radio" id="portrait8" name="portrait" value="portrait8" onChange={props.handleChange} />
                            <img src="https://via.placeholder.com/140x180/" />
                        </label>
                    </div>
                </div>
            </div>
        </div>
    );
}


export default Step3;