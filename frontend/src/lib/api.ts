import axios from 'axios';

async function getUser() {
  const response = await axios.get('/user?ID=12345');
  console.log(response);
}
