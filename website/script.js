const API_URL = 'http://localhost:3000';

document.getElementById('login-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const username = document.getElementById('login-username').value;
  const password = document.getElementById('login-password').value;

  try {
    const response = await fetch(`${API_URL}/login?then_success=https://google.com&then_error=https://www.facebook.com`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });

    const result = await response.json();

    if (response.ok) {
      alert('Login successful!');
      console.log('Token:', result.token); // Token can be stored securely.
    } else {
      document.getElementById('login-message').textContent = result.error || 'Login failed';
    }
  } catch (error) {
    console.error('Error:', error);
    document.getElementById('login-message').textContent = 'Something went wrong.';
  }
});

document.getElementById('signup-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const username = document.getElementById('signup-username').value;
  const password = document.getElementById('signup-password').value;

  try {
    const response = await fetch(`${API_URL}/signup`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });

    const result = await response.json();

    if (response.ok) {
      alert('Signup successful!');
    } else {
      document.getElementById('signup-message').textContent = result.error || 'Signup failed';
    }
  } catch (error) {
    console.error('Error:', error);
    document.getElementById('signup-message').textContent = 'Something went wrong.';
  }
});

