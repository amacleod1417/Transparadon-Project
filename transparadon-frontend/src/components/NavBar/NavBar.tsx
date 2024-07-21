import React, { useState } from 'react'
import logo from '../../assets/dino.svg'
import menu from '../../assets/menu.png'

const NavBar = () => {
  return (
    <div className = 'header'>
        <div className = 'logo-wrap'>
            <img src={logo} alt='' className='logo'/>
        </div>
    <nav>
        <div className = 'links-container'>
          <ul className = 'link'>
            <a href=''>Home</a>
          </ul>
          <ul className = 'link'>
            <a href=''>Community Fund</a>
          </ul>
          <ul className = 'link'>
            <a href=''>Charities</a>
          </ul>
          <ul className = 'link'>
            <a href=''>Impact Tracking</a>
          </ul>
        </div>
        <div className = 'menu-button'>
          <a href=''><img src={menu} alt='' className='menu' width={40}/></a>
        </div>
    </nav>
    </div>
  )
}

export default NavBar