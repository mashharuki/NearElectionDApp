import React from "react";
import './../css/global.css';

/**
 * title component
 * @param {*} props 
 * @returns 
 */
const Title = (props) => {
    return (
        <span className="box-content h-7 w-1/3 p-4 border-4 m4 center text-4xl items-center">
            {props.name}
        </span>
    )
}

export default Title;