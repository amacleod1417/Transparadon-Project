const express = require('express');
const { check, validationResult } = require('express-validator');
const auth = require('../middleware/authMiddleware');
const { register, login, getUser } = require('../controllers/authController');

const router = express.Router();

// @route    POST api/users/register
// @desc     Register user
// @access   Public
router.post(
  '/register',
  [
    check('email', 'Please include a valid email').isEmail(),
    check('password', 'Please enter a password with 6 or more characters').isLength({ min: 6 }),
  ],
  (req, res) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(400).json({ errors: errors.array() });
    }
    register(req, res);
  }
);

// @route    POST api/users/login
// @desc     Authenticate user & get token
// @access   Public
router.post(
  '/login',
  [
    check('email', 'Please include a valid email').isEmail(),
    check('password', 'Password is required').exists(),
  ],
  (req, res) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(400).json({ errors: errors.array() });
    }
    login(req, res);
  }
);

// @route    GET api/users
// @desc     Get user by token
// @access   Private
router.get('/', auth, (req, res) => {
  getUser(req, res);
});

module.exports = router;
