package main

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"os"

	"github.com/charmbracelet/log"

	flag "github.com/spf13/pflag"
)

/*
	"sector": [1000,23],
    "proof_type": 5,
    "commit1_out":
*/

type PostBody struct {
	Sector     []int  `json:"sector"`
	ProofType  int    `json:"proof_type"`
	Commit1Out []byte `json:"commit1_out"`
}

const (
	Host = "http://127.0.0.1:3457"
)

var request PostBody
var file string

func init() {
	flag.IntSliceVar(&request.Sector, "sector", []int{1000, 23}, "help message for sector")
	flag.IntVar(&request.ProofType, "proof_type", 5, "help message for sector")
	flag.StringVarP(&file, "file", "f", "", "commit1_out file")
	flag.Parse()
}

func main() {

	commit1_out, err := os.ReadFile(file)
	if err != nil {
		panic(err)
	}

	request.Commit1Out = commit1_out

	bodyJson, err := json.Marshal(request)
	if err != nil {
		panic(err)
	}

	var body = bytes.NewBuffer(bodyJson)

	resp, err := http.Post(Host+"/remote/seal/commit2", "Application/json", body)

	if err != nil {
		panic(err)
	}

	log.Info("resp", "status", resp.StatusCode, "headers", resp.Header)

	respbody, err := io.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}

	log.Info("respbody", "length", len(respbody), "body", string(respbody))
}
