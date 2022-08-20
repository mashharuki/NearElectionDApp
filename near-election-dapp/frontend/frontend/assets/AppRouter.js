import React from "react";
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import Home from "./pages/home";
import Candidate from "./pages/candidate";
import Voter from "./pages/voter";

/**
 * AppRouter component
 */
const AppRouter = () => {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" exact element={<Home />} />
                <Route path="/candidate" exact element={<Candidate />} />
                <Route path="/voter" exact element={<Voter />} />
            </Routes>
        </BrowserRouter>
    );
}

export default AppRouter;