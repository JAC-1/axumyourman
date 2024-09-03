const campusDropdown = document.querySelector(".dropdown");

campusDropdown.addEventListener("mouseover", (e) => {
  gsap.to(".dropdown-content", {
    opacity: 1,
    y: 0,
    duration: 1.2,
    ease: "power3.out",
    stagger: 0.2,
  });
});
