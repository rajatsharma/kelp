package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use: "init",
	Run: func(cmd *cobra.Command, args []string) {
		dirname, err := os.UserHomeDir()
		if err != nil {
			panic(err)
		}

		_ = os.Mkdir(dirname+"/.kelp", 0755)

		fmt.Println(`
            for file in ~/.kelp/*.fish
                [ -r "$file" ] && [ -f "$file" ] && source "$file";
            end
        `)
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}
