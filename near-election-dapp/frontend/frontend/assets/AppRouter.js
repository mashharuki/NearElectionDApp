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
                <Route path="/" element={<Home />} />
                <Route path="/candidate" element={<Candidate />} />
                <Route path="/voter" element={<Voter />} />
            </Routes>
        </BrowserRouter>
    )
}

export default AppRouter;