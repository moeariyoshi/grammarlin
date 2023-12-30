import React, { useState, useEffect } from 'react';
import axios from './axios/axios';
import './App.css';

function App() {
  const [predictedSuffix, setSuffix] = useState("");
  const [inputValue, setInputValue] = useState("");

  const handleChange = async (e) => {
    const newInputValue = e.target.value;
    setInputValue(newInputValue);

    const words = newInputValue.split(/\s+/);
    const newPrefix = words[words.length - 1];

    try {
      const response = await axios.get(
        '/api/suggestions', 
        { 
          params: { prefix: newPrefix }, // Use 'params' for query parameters
        },
        { 
          headers: {
            'Content-Type': 'application/json',
          }
        }
      );
      console.log(response);
      if (response.data.suggestions.length !== 0 ) {
        setSuffix(response.data.suggestions[0]);
      } else {
        setSuffix("");
      }
    } catch (error) {
      console.error('Error fetching suggestion:', error);
    }
  }

  useEffect(() => {
    setInputValue((prevInputValue) => `${prevInputValue}${predictedSuffix}`);
  }, [predictedSuffix]);

  return (
    <div className="App">
      <header>
        <h1> Welcome to Grammarlin! </h1>
      </header>
      <form>
        <div className="container">
          <div className="form-group">
            <textarea
              className="form-control"
              id="input"
              rows="25"
              placeholder="Let me help you write..."
              onChange={handleChange}
              value={inputValue}
            ></textarea>
          </div>
        </div>
      </form>
    </div>
  );
}

export default App;
