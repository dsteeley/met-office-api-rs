# \SpotdataThreeHourlyApiApi

All URIs are relative to *https://api-metoffice.apiconnect.ibmcloud.com/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_three_hourly_forecast_using_get1**](SpotdataThreeHourlyApiApi.md#get_three_hourly_forecast_using_get1) | **GET** /forecasts/point/three-hourly | Find nearest forecast data that matches the request criteria



## get_three_hourly_forecast_using_get1

> crate::models::SpotForecastFeatureCollection get_three_hourly_forecast_using_get1(latitude, longitude, exclude_parameter_metadata, include_location_name)
Find nearest forecast data that matches the request criteria

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**latitude** | **f64** | The request latitude value in range -85/+85 expressed as a decimal fraction. | [required] |
**longitude** | **f64** | The request longitude value in range -180/+180 expressed as a decimal fraction. | [required] |
**exclude_parameter_metadata** | Option<**bool**> | If true, exclude parameter metadata in the response. If false or undefined, the parameter metadata is returned. |  |
**include_location_name** | Option<**bool**> | If true, include location name in the response. If false or undefined, no location name is returned. |  |

### Return type

[**crate::models::SpotForecastFeatureCollection**](SpotForecastFeatureCollection.md)

### Authorization

[client-id](../README.md#client-id), [client-secret](../README.md#client-secret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

