import NavBar from './components/NavBar/NavBar'
import Home from './components/Home/Home'
import { createRoot } from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
  Route,
  Link,
} from "react-router-dom";

const App = () => {
  return (
    <div className = 'wrapper'>
      <NavBar/>
      <Home/>
    </div>
  )
}

export default App