import React from 'react'
import donation1 from '../../assets/donation1.svg'

const Home = () => {
  return (
    <div className = 'titles'>
        <h1 className = 'title1'>
            Quick Donation
        </h1>
        <ul className = 'donationbanner'>
          <a href = 'https://give.feedingamerica.org/a/donate' target='_blank'>
            <img src={donation1} alt='' className='donation1'/>          
          </a>
        </ul>
        <h2 className = 'title2'>
            Welcome to Transparadon!
        </h2>
        <h4 className = 'title3'>
          Empowering, Giving, and Ensuring Transparency.
        </h4>
    </div>
  )
}

export default Home