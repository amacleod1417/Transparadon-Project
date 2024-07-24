import NavBar from './components/NavBar/NavBar'
import Home from './components/Home/Home'
import { createRoot } from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
  Route,
  Link,
} from "react-router-dom";
import Donate from './components/Pages/Donate';
import { useState } from 'react';

function App (){

  const [page, setPage] = useState('home');
  console.log(`Page: ${page} `)
  return (
    <div className = 'wrapper'>
      <NavBar setPage={setPage}/>

    {page === "home" && <Home/>}

      {page === "donate" && <Donate/>}
  

    </div>
  )
}

export default App