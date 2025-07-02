package util

import (
	"os"

	"github.com/olekukonko/tablewriter"
)

func PrintTable(header []string, rows [][]string) {
	table := tablewriter.NewWriter(os.Stdout)
	table.Header(header)
	table.Bulk(rows)
	table.Render()
}
