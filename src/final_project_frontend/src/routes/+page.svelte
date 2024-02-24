<script>
  import "../index.scss";
  import {backend} from "$lib/canisters";

  let showLogin = true;

  function toggleForm() {
    showLogin = !showLogin;
  }

  function handleSubmit(event) {
    event.preventDefault();
    if (showLogin) {
      onLogin(event);
    } else {
      onSignUp(event);
    }
  }
  
  function onSignUp(event){
      const username = event.target.signup_username.value;
      const password = event.target.signup_password.value;

      backend.sign_up(username, password).then((res) => {
        if (res.Err){
          alert("User already exists.");
        }
        else {
          alert("Signup succesfull. You can login now !.")
          console.log(res);
        }
      });
  }

  function onLogin(event){
      const username = event.target.login_username.value;
      const password = event.target.login_password.value;

      backend.login(username, password).then((res) => {
        if (res.Err){
          alert("Login failed.");
          window.location.href = "/home";
        }
        else {
          console.log(res);
          window.location.href = "/home";
        }
        }); 
      }
</script>
<main>
  <div class="formContainer">
    <div class="toggleButtons">
      <button on:click={toggleForm} class="{showLogin ? 'active' : ''}">Login</button>
      <button on:click={toggleForm} class="{showLogin ? '' : 'active'}">Sign up</button>
    </div>
    <form on:submit={handleSubmit}>
      <h3>{showLogin ? "Login Now!" : "Sign Up!"}</h3>
      <div class="{showLogin ? 'loginContainer' : 'signupContainer'}">
        <input id="{showLogin ? 'login_username' : 'signup_username'}" placeholder="Enter your username" alt="Name" type="text" />
        <input id="{showLogin ? 'login_password' : 'signup_password'}" placeholder="Enter your password" alt="Password" type="password" />
        <button type="submit">{showLogin ? "Login" : "Sign up"}</button>
      </div>
    </form>
  </div>
</main>

<style>
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #f9f9f9;
  }

  .formContainer {
    background-color: #fff;
    border-radius: 10px;
    padding: 30px;
    box-shadow: 0px 0px 20px 0px rgba(0,0,0,0.1);
    max-width: 400px;
    width: 100%;
  }

  .toggleButtons {
    display: flex;
    justify-content: center;
    margin-bottom: 20px;
  }

  .toggleButtons button {
    padding: 10px 20px;
    margin: 0 10px;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
    background-color: #007bff;
    color: #fff;
    transition: all 0.3s ease;
  }

  .toggleButtons button.active {
    background-color: #0056b3;
  }

  .formContainer h3 {
    text-align: center;
    margin-bottom: 20px;
    color: #333;
    font-size: 24px;
  }

  .formContainer input[type="text"],
  .formContainer input[type="password"],
  .formContainer button {
    margin-bottom: 15px;
    padding: 15px;
    border: none;
    border-radius: 5px;
    width: 100%;
    box-sizing: border-box;
    font-size: 16px;
    background-color: #f3f3f3;
    color: #333;
  }

  .formContainer input[type="text"]::placeholder,
  .formContainer input[type="password"]::placeholder {
    color: #999;
  }

  .formContainer button {
    background-color: #007bff;
    color: #fff;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .formContainer button:hover {
    background-color: #0056b3;
  }
</style>