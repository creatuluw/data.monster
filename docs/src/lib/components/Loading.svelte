<div class="loader06"></div>

<style>
  .loader06 {
    width: 112px;
    height: 112px;
    border: 2px solid transparent;
    border-radius: 50%;
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    margin: 0;
  }

  .loader06::before,
  .loader06::after {
    content: '';
    position: absolute;
    border-radius: 50%;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
  
  .loader06::before {
    border: 8px solid rgba(100, 100, 100, 0.5);
    width: 134.4px;
    height: 134.4px;
    animation: loader-scale 1s ease-out infinite;
    animation-delay: 1s;
    opacity: 0;
  }

  .loader06::after {
    border: 4px solid #555;
    width: 112px;
    height: 112px;
    animation: loader-scale 1s ease-out infinite;
    animation-delay: 0.5s;
  }

  @keyframes loader-scale {
    0% {
      transform: translate(-50%, -50%) scale(0);
      opacity: 0;
    }
    50% {
      opacity: 1;
    }
    100% {
      transform: translate(-50%, -50%) scale(1);
      opacity: 0;
    }
  }
</style>
