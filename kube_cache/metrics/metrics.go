package metrics

import (
	"fmt"
	"sync"
)

type Metrics struct {
	Lock                   sync.Mutex
	CacheHits, CacheMisses int
	ImagesRetrieved        int
	DalleRequests          int
	GptRequests            int
	DalleErrors            int
	GptErrors              int
}

func New() *Metrics {
	return &Metrics{}
}

func (m *Metrics) IncrementCacheHits() {
	m.Lock.Lock()
	defer m.Lock.Unlock()
	m.CacheHits++
}

func (m *Metrics) IncrementCacheMisses() {
	m.Lock.Lock()
	defer m.Lock.Unlock()
	m.CacheMisses++
}

func (m *Metrics) IncrementImagesRetrieved() {
	m.Lock.Lock()
	defer m.Lock.Unlock()
	m.ImagesRetrieved++
}

func (m *Metrics) IncrementDalleRequests() {
	m.Lock.Lock()
	defer m.Lock.Unlock()
	m.DalleRequests++
}

func (m *Metrics) IncrementGptRequests() {
	m.Lock.Lock()
	defer m.Lock.Unlock()
	m.GptRequests++
}

func (m *Metrics) IncrementDalleErrors() {
	m.Lock.Lock()
	defer m.Lock.Unlock()
	m.DalleErrors++
}

func (m *Metrics) IncrementGptErrors() {
	m.Lock.Lock()
	defer m.Lock.Unlock()
	m.GptErrors++
}

func (m *Metrics) String() string {
	return fmt.Sprintf(`kube_cache_hits=%d
  kube_cache_misses=%d
  kube_images_retrieved=%d
  kube_dalle_requests=%d
  kube_gpt_requests=%d
  kube_dalle_errors=%d
  kube_gpt_errors=%d`, m.CacheHits, m.CacheMisses, m.ImagesRetrieved, m.DalleRequests, m.GptRequests, m.DalleErrors, m.GptErrors)
}
