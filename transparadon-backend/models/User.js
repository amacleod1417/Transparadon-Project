//user schema

import { Schema, model } from 'mongoose';

const UserSchema = new Schema({
  email: {
    type: String,
    required: true,
    unique: true,
  },
  password: {
    type: String,
    required: true,
  },
  date: {
    type: Date,
    default: Date.now,
  },
    address: {type: String, required: true },

    votingPower: { type: Number, default: 0 }
});

export default model('user', UserSchema);
