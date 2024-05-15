"use client"
import dayjs from 'dayjs'
import * as echarts from 'echarts/core';
import {
  TooltipComponent,
  TooltipComponentOption,
  GridComponent,
  GridComponentOption
} from 'echarts/components';
import { LineChart, LineSeriesOption } from 'echarts/charts';
import { UniversalTransition } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import { getPointTwo, formatBigNumberToletter } from '@/utils/utils';
import { $clientGet } from '@/utils/request'
import { useEffect } from 'react'

echarts.use([
  TooltipComponent,
  GridComponent,
  LineChart,
  CanvasRenderer,
  UniversalTransition
]);

type EChartsOption = echarts.ComposeOption<
  TooltipComponentOption | GridComponentOption | LineSeriesOption
>;
let chartDom: HTMLElement
let myChart: echarts.ECharts
let option: EChartsOption;
option = {
  xAxis: {
    type: 'category',
    splitLine: { show: false },
    boundaryGap: false,
    data: [],
    inverse: true 
  },
  grid: {
    right: '4%',
    bottom: 0,
    top: 10,
    left: 0,
    containLabel: true
  },
  yAxis: {
    type: 'value',
    position: 'right',
    splitLine: { show: false },
  },
  series: [
    {
      data: [],
      type: 'line',
      symbol: 'none',
      areaStyle: {
        color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
          {
            offset: 0,
            color: '#81D8CF'
          },
          {
            offset: 1,
            color: '#06061A'
          }
        ])
      },
      lineStyle: {
        color: '#81D8CF'
      }
    }
  ],
  tooltip: {
    axisPointer: {
      type: 'cross'
    }
  }
};

export default function chart1() {
  // console.log(dayjs(1706170057000).format('YYYY-MM-DD HH:mm:ss'));
  const initCharts = () => {
    chartDom = document.getElementById('main-chart')!;
    myChart = echarts.init(chartDom);
    option && myChart.setOption(option);
    window.addEventListener("resize", function () {
      myChart.resize();
    });
  }

  const getCharts = async () => {
    const res = await $clientGet(`/accounts?page=1&limit=100`)
    const priceList = res.data.map((ele: any) => Number(ele.price))
    const timeList = res.data.map((ele: any) => dayjs(ele.timestamp * 1000).format('HH:mm:ss'))
    const trueMax = Math.max(...priceList)
    const trueMin = Math.min(...priceList)
    const differ = (trueMax - trueMin) / 10
    const max = getPointTwo((trueMax + differ).toString())
    const min = getPointTwo((trueMin - differ).toString())
    myChart.setOption({
      xAxis: {
        data: timeList
      },
      yAxis: {
        max: max,
        min: min,
        axisLabel: {
          formatter: function (value: any) {
            return formatBigNumberToletter(value)
          }
        }
      },
      series: [
        {
          data: priceList,
        }
      ]
    })
  }

  useEffect(() => {
    initCharts()
    getCharts()
    return () => {
      myChart.dispose()
      window.removeEventListener("resize", function () {
        myChart.resize();
      });
    }
  }, [])

  return (
    <div className="w-full h-[270px]" id="main-chart"></div>
  )
}
