const config = {
  pipelines: [
    {
      name: "test",
      steps: [
        {
          name: "mystep",
          commands: ["ls", "echo $0"],
        },
      ],
    },
    {
      name: "test_duplicate",
      steps: [
        {
          name: "mystep",
          commands: ["ls", "echo $0"],
        },
      ],
    },
    {
      name: "test2",
      steps: [
        {
          name: "mystep",
          commands: ["ls"],
        },
      ],
    },
  ],
};
export default config;
