import React, { useState } from 'react'
import logo from '../../assets/dino.svg'
import menu from '../../assets/menu.png'

interface NavBarProps {
  setPage:  (page: string) => void;
};
const NavBar = ({setPage}: NavBarProps) => {

  return (
    <div className = 'header'>
        <div className = 'logo-wrap'>
            <img src={logo} alt='' className='logo'/>
        </div>
    <nav>
        <div className = 'links-container'>
          <ul className = 'link'>
            <div onClick={() => setPage("home")}>Home</div>
          </ul>
          <ul className = 'link'>
            <div onClick={() => setPage("donate")}>Community Fund</div>
          </ul>
          <ul className = 'link'>
            <div onClick={() => setPage("charities")}>Charities</div>
          </ul>
          <ul className = 'link'>
            <div onClick={() => setPage("impact")}>Impact Tracking</div>
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