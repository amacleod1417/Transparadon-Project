import NavBar from './components/NavBar/NavBar'
import Home from './components/Home/Home'
import Footer from './components/Footer/Footer'

const App = () => {
  return (
    <div className = 'wrapper'>
      <NavBar/>
      <Home/>
      <Footer/>
    </div>
  )
}

export default App