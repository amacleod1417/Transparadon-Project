import React from 'react'
import donation1 from '../../assets/donation1.svg'
import donation2 from '../../assets/donation2.png'
import logo from '../../assets/logo.svg'
import Footer from '../../components/Footer/Footer'

const Home = () => {
  return (
    <div className = 'home'>
      <div className = 'titles'>
        <h1 className = 'title1'>
            Quick Donations!
        </h1>
        <ul className = 'donations'>
          <div className = 'charity1'>
            <a href = 'https://give.feedingamerica.org/a/donate' target='_blank'>
              <img src={donation1} alt='' className='donation1'/>          
            </a>
          </div>
          <div className = 'charity2'>
            <a href = 'https://give.thetrevorproject.org/give/599352/#!/donation/checkout' target='_blank'>
              <img src={donation2} alt='' className='donation2'/>
            </a>
          </div>
        </ul>
        <h2 className = 'title2'>
          Empowering, Giving, and Ensuring Transparency.
        </h2>
        <h4 className = 'title3'>
          Join us in revolutionizing charitable donations through blockchain technology where real differences are made. Start exploring our extensive network of vetted charities today and make a meaningful difference in the world. 
          Together, let's make every donation count.
        </h4>
        <div className = 'marker'>
          <img src={logo} alt='' className='logo'/>
        </div>
      </div>
      <Footer/>
    </div>
  )
}



export default Home