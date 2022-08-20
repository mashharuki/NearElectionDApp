import React, { useEffect, useState } from "react";
import {
    nft_transfer, 
    nft_add_likes_to_candidate, 
    nft_tokens_for_kind,
    nft_return_candidate_likes, 
    check_voter_has_been_added, 
    check_voter_has_voted,
    voter_voted, 
    close_election, 
    if_election_closed, 
    reopen_election
} from '../js/near/utils'
import CandidateCard from "../components/candidate_card";
import LikeIcon from '../img/like_icon.png'

/**
 * Home component
 */
const Home = () => {
    // state variable
    const [candidateInfoList, setCandidateInfoList] = useState();
    const [candidateLikesList] = useState([]);
    const [state, setState] = useState("fetching");

    // enum of state
    const State = {
        Fetching: "fetching",
        Fetched: "fetched",
        Open: "open",
        Closed: "closed"
    }

    // hook
    useEffect(async () => {
        // call nft_tokens_for_kind function
        await nft_tokens_for_kind("candidate").then(value => {
            setCandidateInfoList(value);
            setState("fetched");
        });
    }, []);

    /**
     * vote function
     */
     const vote = (token_id) => {
        //check if user has already voted
        check_voter_has_voted(window.accountId).then(value => {
            if (Boolean(value)) {
                alert("You have already voted!")
                return
            }

            // check if user has vote ticket
            check_voter_has_been_added(window.accountId).then(value => {
                let tokenIdOfVoter = parseFloat(value);
                if (tokenIdOfVoter == 0) {
                    alert("You don't have vote ticket! Please ask deployer to give it to you.")
                    return
                }
                // confirm if user really vote to specified candidate
                let isSure = confirm("Once you vote, you can't change selected candidate. Are you OK?");
                if (!isSure) {
                    return
                }
                // execute transfer function
                nft_transfer(process.env.CONTRACT_NAME, tokenIdOfVoter);
                // add vote to specified candidate
                nft_add_likes_to_candidate(token_id);
                //add user ID to voted-list
                voter_voted(window.accountId);
            })
        })
    }

    /**
     * cardsInCaseOpen component
     */
    const cardsInCaseOpen = () => {
        let candidateCardList = [];
        for (let i = 0; i < candidateInfoList.length; i++) {
            // format data for rendering
            candidateCardList.push(
                <div className="items-center">
                    <CandidateCard 
                        CID={candidateInfoList[i].metadata.media_CID} 
                        name={candidateInfoList[i].metadata.candidate_name} 
                        manifest={candidateInfoList[i].metadata.candidate_manifest} 
                    />
                    <div className="center text-xl items-center">
                        <img src={LikeIcon} className="object-cover h-5 w-5 mr-2" />
                        <p className="mr-2">{(candidateLikesList[i])}</p>
                        <button 
                            value={candidateInfoList[i].metadata.token_id} 
                            onClick={(event) => vote(parseInt(event.target.value))} 
                            className="vote_button hover:skew-1"
                        >
                            Vote!
                        </button>
                    </div>
                </div>
            )
        }
        return candidateCardList
    }

    /**
     * cardsInCaseClosed component
     */
    const cardsInCaseClosed = () => {
        let candidateCardList = [];
        let mostVotedNum = candidateLikesList.reduce((a, b) => { return Math.max(a, b) });
        // format data for rendering
        for (let i = 0; i < candidateInfoList.length; i++) {
            if (candidateLikesList[i] == mostVotedNum) {
                // for winner candidate rendering
                candidateCardList.push(
                    <div className="items-center">
                        <div className="text-2xl shadow-rose-600 center font-semibold text-red-700">Won!</div>
                        <CandidateCard 
                            CID={candidateInfoList[i].metadata.media_CID} 
                            name={candidateInfoList[i].metadata.candidate_name} 
                            manifest={candidateInfoList[i].metadata.candidate_manifest} 
                        />
                        <div className="center text-xl items-center">
                            <img src={LikeIcon} className="object-cover h-5 w-5 mr-2" />
                            <p className="mr-2">{(candidateLikesList[i])}</p>
                        </div>
                    </div>

                )
            } else {
                // for other candidate rendering
                candidateCardList.push(
                    <div className="items-center opacity-20">
                        <div className="pt-7"></div>
                        <CandidateCard 
                            CID={candidateInfoList[i].metadata.media_CID} 
                            name={candidateInfoList[i].metadata.candidate_name} 
                            manifest={candidateInfoList[i].metadata.candidate_manifest} 
                        />
                        <div className="center text-xl items-center">
                            <img src={LikeIcon} className="object-cover h-5 w-5 mr-2" />
                            <p className="mr-2">{(candidateLikesList[i])}</p>
                        </div>
                    </div>

                )
            }

        }
        return candidateCardList
    }

    /**
     * getCandidateLikes function 
     */
    const getCandidateLikes = async () => {
        // get num of likes for each candidate
        for (let i = 0; i < candidateInfoList.length; i++) {
            // execute nft_return_candidate_likes function
            await nft_return_candidate_likes(candidateInfoList[i].metadata.token_id).then(value => {
                candidateLikesList.push(value);
            })
        }

        // check if election is closed
        let isClosed = await if_election_closed();
        console.log(isClosed);

        if (isClosed) {
            setState("closed");
        } else {
            setState("open")
        }
    }

    /**
     * Close button component
     */
    const closeButton = () => {
        // check if user is contract deployer
        if (window.accountId !== process.env.CONTRACT_NAME) {
            return
        }

        return (
            <button 
                className="close_button hover:skew-1 h-10 bg-red-600 mb-3" 
                onClick={() => {
                    // confirm that user really close this election
                    let isSureToClose = confirm("Are you sure to close this election?");
                    if (isSureToClose) {
                        // execute close_election function
                        close_election()
                        // change state to closed
                        setState("closed")
                    }
                }}
            >
                Close Election
            </button>
        );
    }

    /**
     * Reopme button component
     */
    const reopenButton = () => {
        // check if user is contract deployer
        if (window.accountId !== process.env.CONTRACT_NAME) {
            return
        }

        return (
            <button 
                className="close_button hover:skew-1 h-10 bg-red-600 mb-3" 
                onClick={() => {
                    let isSureToClose = confirm("Are you sure to reopen this election?");
                    if (isSureToClose) {
                        // execute reopen_election function
                        reopen_election()
                        // change state to open
                        setState("open")
                    }
                }}
            >
                Reopen Election
            </button>
        );
    }

    /**
     * messageToWait component
     */
    const messageToWait = () => {
        return <div className="grid h-screen place-items-center text-3xl">Fetching NFTs of candidates...</div>
    }

    switch (state) {
        case State.Fetching:
            return <div>{messageToWait()}</div>
        case State.Fetched:
            // get likes
            getCandidateLikes();
            return <div>{messageToWait()}</div>
        case State.Open:
            return (
                <div>
                    <div className="center">{closeButton()}</div>
                    <div className="grid grid-cols-3 gap-10">
                        {cardsInCaseOpen()}
                    </div>
                </div>
            )
        case State.Closed:
            return (
                <div>
                    <div className="center">{reopenButton()}</div>
                    <div className="grid grid-cols-3 gap-10">
                        {cardsInCaseClosed()}
                    </div>
                </div>
            )
    }
}

export default Home;