import React from 'react';
import Tooltip from 'react-bootstrap/Tooltip'
import Popover from 'react-bootstrap/Popover'
import OverlayTrigger from 'react-bootstrap/OverlayTrigger'
import Badge from 'react-bootstrap/Badge'

const Step2 = (props) => {
    if (props.currentStep !== 2) {
        return null
    }

    const popover = (
        <Popover id="popover-basic">
          <Popover.Title as="h3" style={{color: 'black'}}>Force</Popover.Title>
          <Popover.Content>
          {props.strenghHelper}
          </Popover.Content>
        </Popover>
      );

      const Example = () => (

        <OverlayTrigger placement="right"  overlay={popover}>
            <Badge pill variant="info">
                i
            </Badge>
        </OverlayTrigger>
      );

    return (
        

        <div className="form-group">
            {/* <div className='row'> */}
                <div class="col-md-12">
                <label htmlFor="strengh">Force</label>
                <div className="float-right">
                <Example />
                </div>
                    <div class="input-group">

                        <button className="btn btn-outline-primary" type="button" name="strengh" value="-10" onClick={props.onclickHandler} >-10</button>
                        <button className="btn btn-outline-primary" type="button" name="strengh" value="-1" onClick={props.onclickHandler} >-1</button>

                        <input
                            className="form-control"
                            id="strengh"
                            name="strengh"
                            type="number"
                            disabled
                            min={props.min_point}
                            max={props.max_point}
                            value={props.strengh}
                        />
                        <button className="btn btn-outline-primary" type="button" name="strengh" value="+1" onClick={props.onclickHandler} >+1</button>
                        <button className="btn btn-outline-primary" type="button" name="strengh" value="+10" onClick={props.onclickHandler}>+10</button>

                    </div>
                    <label htmlFor="dexterity">Dexterité</label>
                    <input
                        className="form-control"
                        id="dexterity"
                        name="dexterity"
                        type="number"
                        min={props.min_point}
                        max={props.max_point}
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
                        min={props.min_point}
                        max={props.max_point}
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
                        min={props.min_point}
                        max={props.max_point}
                        value={props.charism}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />
                {/* </div> */}

                {/* <div class="col-md-6"> */}
                    <label htmlFor="perception">Perception</label>
                    <input
                        className="form-control"
                        id="perception"
                        name="perception"
                        type="number"
                        min={props.min_point}
                        max={props.max_point}
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
                        min={props.min_point}
                        max={props.max_point}
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
                        min={props.min_point}
                        max={props.max_point}
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
                        min={props.min_point}
                        max={props.max_point}
                        value={props.education}
                        onChange={props.handleChange}
                        onFocus={props.handleFocus}
                    />
                </div>
            </div>
        // </div>
    );
}

export default Step2;